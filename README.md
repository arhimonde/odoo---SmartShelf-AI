# 🚀 SmartShelf AI — Sistema de Retail Autónomo

[![Odoo](https://img.shields.io/badge/Odoo-17.0-purple.svg)](https://www.odoo.com/)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Jetson](https://img.shields.io/badge/NVIDIA-Jetson-green.svg)](https://www.nvidia.com/en-us/autonomous-machines/embedded-systems/)

Sistema avanzado de monitorización de inventario en tiempo real que utiliza **Visión por Computadora (AI)** en el borde (Edge Computing) y sincronización automática con **Odoo ERP**.

---

## 🧐 ¿Qué es SmartShelf AI?

SmartShelf AI transforma estantes convencionales en estantes inteligentes. Utilizando una cámara y una NVIDIA Jetson, el sistema detecta automáticamente cuando un cliente toma o repone un producto, actualizando el stock en la nube sin intervención humana.

## 🛠️ Características Principales

*   **Procesamiento en el Borde (Edge AI)**: Ejecuta modelos YOLOv8 directamente en la Jetson para una latencia mínima.
*   **Motor de Tracking en Rust**: Implementación ultra-rápida de algoritmos de geometría 2D para detectar entradas y salidas de la zona de interés (ROI).
*   **Sincronización Inteligente**: Comunicación asíncrona con Odoo mediante una cola de mensajes que garantiza que no se pierda ninguna actualización, incluso con caídas de red (**Retry Logic**).
*   **Logs Detallados**: Sistema de trazabilidad completo mediante `tracing` para monitorizar cada transacción.

## 📐 Arquitectura del Sistema

1.  **Cámara / YOLOv8**: Captura el flujo de video y genera coordenadas de los objetos.
2.  **Rust Engine (`shelf_tracker.rs`)**: Analiza si el objeto está "DENTRO" o "FUERA" del estante virtual.
3.  **Odoo Client (`odoo_client.rs`)**: Envía los eventos (Taken/Restocked) al ERP.

## 📦 Estructura del Repositorio

*   `/src`: Código fuente del motor en Rust.
*   `/odoo_addon`: Módulo personalizado para Odoo que recibe las actualizaciones de stock.
*   `Cargo.toml`: Configuración de dependencias (Tokio, Reqwest, Serde).

## 🚀 Instalación y Uso

### 1. Preparación en Jetson
Asegúrate de que el módulo Odoo en `/odoo_addon` esté instalado en tu servidor Odoo.

### 2. Ejecutar el Motor (Rust)
```bash
# Compilar y ejecutar
cargo run
```

---
Desarrollado para optimizar la logística y el retail moderno. 🚀
