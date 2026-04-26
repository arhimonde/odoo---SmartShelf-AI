use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Self {
        Self { vertices }
    }

    /// Implementare Ray Casting pentru a verifica dacă un punct este în interiorul poligonului
    pub fn contains_point(&self, point: &Point) -> bool {
        let mut inside = false;
        let n = self.vertices.len();
        if n < 3 {
            return false;
        }

        let mut j = n - 1;
        for i in 0..n {
            let vi = &self.vertices[i];
            let vj = &self.vertices[j];

            if ((vi.y > point.y) != (vj.y > point.y))
                && (point.x < (vj.x - vi.x) * (point.y - vi.y) / (vj.y - vi.y) + vi.x)
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }
}

pub struct Detection {
    pub id: u32,
    pub class_name: String,
    pub bbox: [f32; 4], // [x1, y1, x2, y2]
}

impl Detection {
    pub fn center(&self) -> Point {
        Point {
            x: (self.bbox[0] + self.bbox[2]) / 2.0,
            y: (self.bbox[1] + self.bbox[3]) / 2.0,
        }
    }
}

pub struct ShelfTracker {
    roi: Polygon,
    object_history: HashMap<u32, bool>, // ID -> was_inside
    event_tx: mpsc::Sender<(String, i32)>,
}

impl ShelfTracker {
    pub fn new(roi: Polygon, event_tx: mpsc::Sender<(String, i32)>) -> Self {
        Self {
            roi,
            object_history: HashMap::new(),
            event_tx,
        }
    }

    pub async fn process_detections(&mut self, detections: Vec<Detection>) {
        for det in detections {
            let center = det.center();
            let is_currently_inside = self.roi.contains_point(&center);
            
            // Verificăm dacă avem istoric pentru acest ID
            if let Some(&was_inside) = self.object_history.get(&det.id) {
                if was_inside && !is_currently_inside {
                    // INSIDE -> OUTSIDE => Item_Taken
                    let _ = self.event_tx.send((det.class_name.clone(), -1)).await;
                    println!("[EVENT] Item Taken: {}", det.class_name);
                } else if !was_inside && is_currently_inside {
                    // OUTSIDE -> INSIDE => Item_Restocked
                    let _ = self.event_tx.send((det.class_name.clone(), 1)).await;
                    println!("[EVENT] Item Restocked: {}", det.class_name);
                }
            }
            
            // Actualizăm istoricul
            self.object_history.insert(det.id, is_currently_inside);
        }
    }
}
