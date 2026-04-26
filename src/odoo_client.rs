use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryPayload {
    pub product_id: u32,
    pub qty_change: i32,
}

pub struct OdooClient {
    pub url: String,
    pub db: String,
    pub username: String,
    pub api_key: String,
    product_map: HashMap<String, u32>,
    queue_tx: mpsc::Sender<InventoryPayload>,
}

impl OdooClient {
    pub fn new(url: String, db: String, username: String, api_key: String) -> (Self, mpsc::Receiver<InventoryPayload>) {
        let mut product_map = HashMap::new();
        product_map.insert("Soda".to_string(), 45);
        product_map.insert("Snacks".to_string(), 102);
        product_map.insert("Water".to_string(), 8);

        let (tx, rx) = mpsc::channel(100);

        let client = Self {
            url,
            db,
            username,
            api_key,
            product_map,
            queue_tx: tx,
        };

        (client, rx)
    }

    pub async fn update_inventory(&self, product_name: &str, quantity_change: i32) {
        if let Some(&product_id) = self.product_map.get(product_name) {
            let payload = InventoryPayload {
                product_id,
                qty_change: quantity_change,
            };
            
            info!(product = product_name, id = product_id, change = quantity_change, "Adăugare eveniment în coada de sincronizare Odoo.");
            
            if let Err(e) = self.queue_tx.send(payload).await {
                error!("Nu s-a putut trimite evenimentul către worker-ul de sync: {}", e);
            }
        } else {
            warn!(product = product_name, "Produsul nu are o mapare corespunzătoare în baza de date Odoo.");
        }
    }

    pub async fn start_sync_worker(
        url: String,
        api_key: String,
        mut rx: mpsc::Receiver<InventoryPayload>,
    ) {
        let client = Client::new();
        let endpoint = format!("{}/api/update_stock", url);
        let mut retry_buffer: Vec<InventoryPayload> = Vec::new();

        info!(endpoint = %endpoint, "Worker-ul de sincronizare Odoo este pregătit.");

        loop {
            tokio::select! {
                Some(payload) = rx.recv() => {
                    retry_buffer.push(payload);
                }
                _ = sleep(Duration::from_secs(5)), if !retry_buffer.is_empty() => {
                    info!(buffer_size = retry_buffer.is_empty(), "Verificare buffer pentru reîncercare sync...");
                }
            }

            let mut failed_this_round = Vec::new();
            
            for payload in retry_buffer.drain(..) {
                info!(product_id = payload.product_id, "Sincronizare stoc cu Odoo Cloud...");
                
                let res = client.post(&endpoint)
                    .header("X-API-KEY", &api_key)
                    .json(&payload)
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.status().is_success() => {
                        info!(product_id = payload.product_id, "Succes! Stoc actualizat în Odoo.");
                    }
                    Ok(resp) => {
                        error!(
                            status = %resp.status(), 
                            product_id = payload.product_id, 
                            "Odoo a respins cererea. Rămâne în buffer pentru retry."
                        );
                        failed_this_round.push(payload);
                    }
                    Err(e) => {
                        error!(
                            error = %e, 
                            product_id = payload.product_id, 
                            "Eroare de rețea la conexiunea cu Odoo. Reîncercare în 10 secunde..."
                        );
                        failed_this_round.push(payload);
                    }
                }
            }

            retry_buffer.extend(failed_this_round);

            if !retry_buffer.is_empty() {
                sleep(Duration::from_secs(10)).await;
            }
        }
    }
}
