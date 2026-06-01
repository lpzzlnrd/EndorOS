# EndorOS — Cómo funciona todo

## Qué es EndorOS

EndorOS es un sistema operativo conceptual escrito en Rust. No es una distribución Linux ni un fork de ningún kernel existente — parte desde `no_std`, lo que significa que no depende de la librería estándar de Rust y construye sus propias abstracciones desde abajo.

La ambientación visual toma como referencia la luna de Endor (Star Wars): paleta neutra (gris piedra, verde musgo desaturado, arena, negro suave), animaciones mínimas y énfasis en legibilidad. La experiencia objetivo es similar a Windows 11 Pro pero sin la sobrecarga.

---

## Stack técnico

| Capa | Tecnología |
|---|---|
| Lenguaje | Rust (edición estable) |
| Entorno de ejecución | `no_std` — sin librería estándar |
| Patrón de diseño | Arquitectura hexagonal (Ports & Adapters) |
| UI | Window Manager + Shell personalizado |

### Por qué Rust

- Seguridad de memoria sin garbage collector.
- Control directo sobre hardware (ideal para kernels).
- Abstracciones de costo cero.
- `no_std` permite compilar sin depender de un sistema operativo subyacente.

---

## Arquitectura hexagonal explicada

EndorOS organiza el código en tres anillos concéntricos. Nada del anillo exterior puede romper el interior — las dependencias solo fluyen hacia adentro.

```
┌─────────────────────────────────────────────┐
│              INTERFACES UI                  │
│         Shell · Window Manager              │  ← Anillo exterior
│                                             │
│  Habla con el núcleo solo via Ports         │
└──────────────────┬──────────────────────────┘
                   │ Application Ports
                   │ (traits/interfaces en Rust)
┌──────────────────▼──────────────────────────┐
│           NÚCLEO DE APLICACIÓN              │
│                                             │  ← Anillo medio
│  Casos de uso:                              │
│  · Gestión de procesos                      │
│  · Sistema de archivos                      │
│  · Red                                      │
│  · Seguridad y sesión                       │
│                                             │
└────────┬─────────────────────────┬──────────┘
         │ Domain Ports            │ Infra Ports
         │                         │
┌────────▼──────────┐   ┌──────────▼──────────┐
│  DOMINIO EndorOS  │   │    ADAPTADORES       │
│                   │   │                      │  ← Anillo interior
│  Reglas puras     │   │  drivers, FS, red,   │
│  Entidades del OS │   │  auth, IPC           │
└───────────────────┘   └──────────────────────┘
```

### Qué es un Port

Un Port es un contrato (trait en Rust) que define qué operaciones existen, sin decir cómo se implementan. Por ejemplo: `FileSystemPort` define `read()`, `write()`, `delete()` — no importa si el backend es ext4, FAT32 o un ramdisk.

### Qué es un Adapter

Un Adapter es la implementación concreta de un Port. Para producción: driver real. Para tests: mock en memoria. El núcleo nunca sabe cuál está usando.

---

## Capas detalladas

### 1. Dominio

Contiene las **entidades y reglas de negocio** del sistema operativo:

- `Proceso` — unidad de ejecución con PID, estado, prioridad.
- `Sesión` — usuario activo, permisos, políticas.
- `Archivo` — representación abstracta de dato persistido.
- `Política de seguridad` — reglas de aislamiento y control de ejecución.

No importa nada de hardware aquí. Es Rust puro.

### 2. Núcleo de Aplicación (Casos de uso)

Orquesta el dominio para cumplir objetivos concretos del OS:

| Caso de uso | Qué hace |
|---|---|
| `LaunchProcess` | Crea un proceso, asigna memoria, notifica al scheduler |
| `MountFilesystem` | Registra un FS disponible via Port |
| `AuthenticateUser` | Valida credenciales, inicia sesión |
| `UpdatePackage` | Descarga, verifica firma, instala transaccionalmente |
| `EncryptVolume` | Delega al adaptador de cifrado (BitLocker-equivalent) |

### 3. Adaptadores de Infraestructura

Implementaciones concretas que hablan con hardware o servicios externos:

- **Driver FS**: lee/escribe sectores del disco.
- **Driver de red**: maneja paquetes IP/Ethernet.
- **Auth adapter**: verifica tokens, contraseñas hasheadas.
- **IPC adapter**: pipes, shared memory, sockets locales.

### 4. Interfaces UI

- **Shell**: intérprete de comandos. Recibe input del usuario, llama Application Ports, imprime output.
- **Window Manager**: gestiona ventanas, layouts, escritorios virtuales. Snap layouts inspirados en Windows 11.

---

## Funcionalidades objetivo y cómo se implementan

### Gestión de ventanas

El Window Manager corre como un proceso privilegiado. Mantiene un árbol de ventanas, escucha eventos de input (teclado/mouse via driver adapter) y redibuja la pantalla con el renderer.

Snap layouts = el WM divide el espacio de pantalla en zonas predefinidas y asigna ventanas a ellas cuando el usuario arrastra hacia un borde.

### Sistema de cuentas y políticas

- Entidad `Usuario` con roles (admin, usuario estándar, invitado).
- `PolicyEngine` en el núcleo evalúa si una operación está permitida antes de ejecutarla.
- Hardening de sesión: tiempo de expiración, bloqueo automático.

### Cifrado de disco

- Port `EncryptionPort` con método `encrypt_volume(device, key)`.
- Adapter concreto implementa AES-256-XTS o ChaCha20-Poly1305.
- Las claves nunca salen del dominio en texto plano.

### Actualizaciones transaccionales

El gestor de paquetes sigue este flujo:

```
Descarga paquete
      │
      ▼
Verifica firma criptográfica
      │
      ▼
Crea snapshot del estado actual (rollback point)
      │
      ▼
Aplica cambios
      │
   ┌──┴──┐
  OK    ERROR
   │      │
Confirma Rollback automático al snapshot
```

### Subsistema de desarrollo

Terminal avanzada con multiplexado de sesiones. Toolchains (Rust, C, C++) disponibles como paquetes firmados. Contenedores ligeros via namespace isolation (análogo a Linux namespaces, implementado desde cero).

### Administrador de paquetes

- Repositorios firmados con clave pública.
- Resolución de dependencias en el núcleo.
- Instalación atómica (todo o nada).

---

## Flujo de arranque (boot sequence)

```
Hardware encendido
      │
      ▼
Bootloader (carga el kernel EndorOS desde disco)
      │
      ▼
Kernel init en no_std
  · Inicializa memoria (heap básico)
  · Registra adaptadores de hardware
  · Arranca el scheduler de procesos
      │
      ▼
Init process (PID 1)
  · Monta sistema de archivos raíz
  · Carga políticas de seguridad
  · Arranca servicios del sistema (red, auth, IPC)
      │
      ▼
Session Manager
  · Muestra pantalla de login
  · Autentica usuario
  · Lanza Window Manager + Shell
      │
      ▼
Escritorio EndorOS operativo
```

---

## Principios de diseño aplicados al código

| Principio | Aplicación concreta |
|---|---|
| Menos fricción | Un solo centro de configuración, no paneles duplicados |
| Configuración clara | Archivos de configuración en formato legible, sin registros opacos |
| Rendimiento predecible | `no_std` elimina overhead de runtime; asignación de memoria explícita |
| Seguridad por defecto | Aislamiento de procesos activado por defecto; principio de mínimo privilegio |

---

## Paleta visual y UX

| Color | Rol |
|---|---|
| Gris piedra | Fondo principal de ventanas y paneles |
| Verde musgo desaturado | Acentos, indicadores de estado activo |
| Arena | Texto secundario, bordes sutiles |
| Negro suave | Barra de tareas, headers |

Las animaciones son intencionales y breves — no decorativas. Un menú aparece en <100ms o no anima.

---

## Estado del proyecto

El repositorio contiene actualmente la **base conceptual y documental**. La implementación incremental en Rust seguirá este orden sugerido:

1. Bootloader mínimo + init en `no_std`.
2. Allocator básico (heap).
3. Scheduler de procesos simple (round-robin).
4. Driver FS mínimo (ramdisk).
5. Shell de texto funcional.
6. Adaptadores de red.
7. Auth y gestión de sesiones.
8. Window Manager + renderer.
9. Cifrado y políticas de seguridad.
10. Gestor de paquetes.
