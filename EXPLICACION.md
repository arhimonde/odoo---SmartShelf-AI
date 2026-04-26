# 🚀 SmartShelf AI — Sistema de Retail Autónomo

Este proyecto ha sido creado para demostrar el poder del procesamiento "Edge" en NVIDIA Jetson, combinando visión por computadora (Vision AI) con la integración en un sistema ERP (Odoo).

## 🧐 ¿Para qué se utiliza?
El sistema monitoriza un estante virtual mediante una cámara de video y detecta automáticamente cuándo los clientes toman o reponen productos. El objetivo es la automatización del inventario eliminando la necesidad de escaneo manual en caja.

## 🛠️ ¿Cómo funciona?

1.  **Vision AI (YOLOv8)**:
    *   El sistema recibe coordenadas (bounding boxes) de objetos desde un flujo de video procesado en la Jetson.
    *   Utiliza un módulo en Rust (`shelf_tracker.rs`) para monitorizar si el centro del objeto está dentro o fuera de una zona predefinida (**ROI - Region of Interest**).

2.  **Detección de Transacciones**:
    *   **ITEM TAKEN (Producto Tomado)**: Si un producto (ej: Refresco) sale de la zona del estante.
    *   **ITEM RESTOCKED (Repuesto)**: Si un producto entra en la zona del estante.

3.  **Sincronización Cloud (Odoo)**:
    *   Cada acción física se envía instantáneamente a Odoo ERP mediante el módulo `odoo_client.rs`.
    *   El sistema cuenta con **Retry Logic**: Si falla la conexión a internet, las peticiones se encolan y se envían automáticamente en cuanto vuelve la conexión.

## 📦 Estructura del Proyecto
*   `src/shelf_tracker.rs`: Lógica de geometría 2D (Point-in-Polygon).
*   `src/odoo_client.rs`: Cliente de comunicación REST con Odoo y worker de sincronización.
*   `src/main.rs`: Runtime asíncrono (Tokio) que orquesta todo el sistema.

## 🚀 ¿Cómo ejecutarlo?
Desde esta carpeta, ejecuta:
```bash
cargo run
```

---
*Proyecto configurado para NVIDIA Jetson & Odoo Cloud.*
