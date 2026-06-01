# EndorOS

Sistema operativo conceptual desarrollado en **Rust** con **arquitectura hexagonal** y una interfaz visual **minimalista** de tonos neutros, inspirado en la luna boscosa de Endor (Star Wars).

## Autor
- **Leonardo Correa**
- **C.I. 30889380**
- **Universidad José Antonio Páez**
- **Escuela de Computación**

## Visión del proyecto
EndorOS busca ofrecer una experiencia moderna tipo escritorio, tomando como referencia las capacidades de productividad de Windows 11 Pro, pero con un enfoque más limpio, estable y sin sobrecarga innecesaria.

## Ambientación: Luna de Endor (Star Wars)
Investigación base para ambientación del sistema:
- Endor (luna santuario) destaca por bosques gigantes, naturaleza densa y vida comunitaria.
- Ambiente sereno pero resiliente: equilibrio entre belleza natural y supervivencia.
- Presencia de aldeas elevadas (Ewoks), diseño orgánico y uso eficiente de recursos.

Traducción de ambientación a UX/UI:
- **Paleta neutra**: grises piedra, verde musgo desaturado, arena y negro suave.
- **Iconografía simple** con énfasis en legibilidad.
- **Interacciones sobrias**: animaciones mínimas, rápidas y funcionales.

## Stack técnico
- **Lenguaje principal:** Rust
- **Kernel base:** `no_std` + capas modulares
- **Diseño:** Patrón hexagonal (Ports & Adapters)

## Arquitectura hexagonal propuesta
```text
                 +---------------------------+
                 |       Interfaces UI       |
                 | (Shell, Window Manager)   |
                 +-------------+-------------+
                               |
                    (Application Ports)
                               |
+------------------------------v------------------------------+
|                    Núcleo de Aplicación                    |
|  Casos de uso: procesos, archivos, red, seguridad, sesión  |
+-------------+------------------------+----------------------+
              |                        |
      (Domain Ports)            (Infra Ports)
              |                        |
   +----------v--------+     +--------v----------------------+
   |   Dominio EndorOS |     |      Adaptadores              |
   | Reglas y entidades|     | drivers, FS, red, auth, IPC   |
   +-------------------+     +-------------------------------+
```

## Funcionalidades objetivo (inspiradas en Windows 11 Pro)
> Objetivo: cubrir capacidades equivalentes de uso profesional, optimizadas para rendimiento y simplicidad.

1. **Gestión moderna de ventanas** (snap layouts, escritorios virtuales, multitarea real).
2. **Sistema de cuentas y políticas** (roles, permisos, hardening de sesión).
3. **Cifrado de disco y archivos** (equivalente funcional a BitLocker).
4. **Administración empresarial** (telemetría opcional, políticas locales/centralizadas).
5. **Subsistema de desarrollo** (terminal avanzada, contenedores ligeros, toolchains Rust/C/C++).
6. **Actualizaciones robustas** (transaccionales, rollback automático).
7. **Seguridad por defecto** (aislamiento de procesos, control de ejecución, auditoría).
8. **Administrador de paquetes** con repos firmados.
9. **Explorador de archivos productivo** y búsqueda rápida indexada.
10. **Centro de configuración unificado** sin paneles duplicados.

## Principios de diseño
- Menos fricción, más productividad.
- Configuración clara y centralizada.
- Rendimiento predecible en hardware modesto.
- Seguridad fuerte sin sacrificar usabilidad.

## Estado actual
Este repositorio define la base conceptual y documental de EndorOS para su implementación incremental en Rust.
