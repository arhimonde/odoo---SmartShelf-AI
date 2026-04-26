mod shelf_tracker;
mod odoo_client;

use shelf_tracker::{Point, Polygon, ShelfTracker, Detection};
use odoo_client::OdooClient;
use tokio::sync::mpsc;
use std::sync::Arc;
use tracing::{info, warn, error, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Inițializăm sistemul de logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Inițializare sistem Autonomous Retail Checkout...");

    // 1. Configurare Odoo Client
    let odoo_url = "http://192.168.1.223:8069".to_string();
    let (odoo, odoo_rx) = OdooClient::new(
        odoo_url.clone(),
        "proiect".to_string(),
        "admin".to_string(),
        "SECRET_API_KEY".to_string()
    );

    let odoo = Arc::new(odoo);
    let odoo_clone = odoo.clone();

    // 2. Pornim worker-ul de sincronizare Odoo (Background Sync & Retry)
    info!("Pornire Odoo Sync Worker la adresa {}...", odoo_url);
    tokio::spawn(async move {
        odoo_client::OdooClient::start_sync_worker(
            odoo_url,
            "SECRET_API_KEY".to_string(),
            odoo_rx
        ).await;
    });

    // 3. Configurare canal pentru evenimente de inventar
    let (event_tx, mut event_rx) = mpsc::channel::<(String, i32)>(100);

    // 4. Definire ROI (Virtual Shelf)
    let shelf_roi = Polygon::new(vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 500.0, y: 0.0 },
        Point { x: 500.0, y: 500.0 },
        Point { x: 0.0, y: 500.0 },
    ]);

    // 5. Inițializare Shelf Tracker
    let mut tracker = ShelfTracker::new(shelf_roi, event_tx);

    // 6. LOOP PRINCIPAL: Ascultăm evenimentele de inventar și actualizăm Odoo
    tokio::spawn(async move {
        info!("Event Dispatcher activ. Așteptare evenimente de la senzori...");
        while let Some((product, quantity_change)) = event_rx.recv().await {
            let event_type = if quantity_change == -1 { "TAKEN" } else { "RESTOCKED" };
            
            info!(
                event = event_type,
                product = %product,
                change = quantity_change,
                "S-a detectat o acțiune fizică pe raft."
            );

            // Sincronizăm instantaneu cu Odoo Cloud
            odoo_clone.update_inventory(&product, quantity_change).await;
            
            info!("Cererea de actualizare stoc pentru '{}' a fost trimisă către coada Odoo.", product);
        }
    });

    // 7. SIMULARE: Primirea datelor de la YOLOv8
    info!("Simulare flux date YOLOv8 pornită...");
    
    // Scenariu: Utilizatorul ia o Soda de pe raft
    info!("SCENARIU: Clientul ia o 'Soda' de pe raft.");
    
    // Obiectul Soda (ID: 10) este detectat în interiorul raftului
    let det_in = vec![Detection { 
        id: 10, 
        class_name: "Soda".to_string(), 
        bbox: [100.0, 100.0, 200.0, 200.0] 
    }];
    tracker.process_detections(det_in).await;

    // Așteptăm puțin pentru a simula mișcarea
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Obiectul Soda (ID: 10) se mută în afara ROI
    let det_out = vec![Detection { 
        id: 10, 
        class_name: "Soda".to_string(), 
        bbox: [600.0, 600.0, 700.0, 700.0] 
    }];
    tracker.process_detections(det_out).await;

    // Scenariu: Angajatul pune un Snack înapoi pe raft
    info!("SCENARIU: Angajatul restochează 'Snacks'.");
    
    // Obiectul Snack (ID: 22) apare întâi în afara raftului
    let det_snack_out = vec![Detection { 
        id: 22, 
        class_name: "Snacks".to_string(), 
        bbox: [800.0, 10.0, 900.0, 100.0] 
    }];
    tracker.process_detections(det_snack_out).await;

    // Apoi se mută în interiorul raftului
    let det_snack_in = vec![Detection { 
        id: 22, 
        class_name: "Snacks".to_string(), 
        bbox: [50.0, 50.0, 150.0, 150.0] 
    }];
    tracker.process_detections(det_snack_in).await;

    // Lăsăm sistemul să proceseze sincronizările
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    info!("Simulare finalizată.");
}
