

# Tracer

Rastreador de incidencias ligero para agentes de IA. Rastrea las dependencias entre tareas y coordina múltiples agentes que trabajan en el mismo proyecto.

## Qué Hace

- Rastrear tareas con dependencias (la tarea B bloquea la tarea A)
- Encontrar trabajo listo para comenzar (sin bloqueos)
- Varios agentes pueden trabajar juntos y dejar comentarios
- Almacenamiento basado en Git (archivos JSONL)

## Instalación

```bash
cargo install --git https://github.com/Abil-Shrestha/tracer
```

Requiere la cadena de herramientas de Rust: https://rustup.rs/

## Uso

```bash
tracer init                                    # Initialize in your project
tracer create "Task name" -p 1 -t feature     # Create issue
tracer ready                                   # See available work
tracer update bd-1 --status in_progress       # Start work
tracer comment bd-1 "Working on this"         # Leave comment
tracer close bd-1                              # Close issue
```

## Coordinación Multiagente

```bash
# Agent 1 starts work
tracer --actor agent-1 update bd-1 --status in_progress
tracer comment bd-1 "Working on auth API"

# Agent 2 sees it
tracer show bd-1  # Shows assignee and comments
tracer comment bd-1 "I'll test it when ready"

# Agent 1 finishes
tracer close bd-1
```

Asigna automáticamente al agente cuando el estado cambia a in_progress. Los comentarios aparecen en `tracer show`.

## Características

- Seguimiento de dependencias (bloqueos, padre-hijo, relacionados, descubiertos-de)
- Coordinación multiagente mediante comentarios y asignación automática
- Salida JSON para agentes de IA (parámetro `--json`)
- Almacenamiento compatible con Git (JSONL)
- Descubre automáticamente la base de datos, al igual que git

## Comandos

```bash
tracer create "Title" [-p priority] [-t type]
tracer list [--status STATUS]
tracer show <id>
tracer update <id> --status STATUS
tracer close <id>
tracer comment <id> "message"
tracer dep add <from> <to> --type TYPE
tracer ready
tracer stats
```

Añade `--json` a cualquier comando para obtener salida en JSON.

## Documentación

- [AGENTS.md](./AGENTS.md) - Guía de integración para agentes de IA
- [MULTI_AGENT.md](./MULTI_AGENT.md) - Coordinación multiagente
- [CHANGELOG.md](./CHANGELOG.md) - Historial de versiones

## Licencia

MIT
