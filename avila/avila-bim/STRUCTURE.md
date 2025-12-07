# Avila BIM - Estrutura de Arquivos

```
avila-bim/
├── src/
│   ├── lib.rs                          # Main library module
│   │
│   ├── bim-core.rs                     # ✅ Core BIM primitives
│   ├── bim-converter.rs                # ✅ RabbitMQ converter worker
│   │
│   ├── ifc/                            # 📦 IFC Parser Module
│   │   ├── mod.rs
│   │   ├── parser.rs                   # STEP-File parser
│   │   ├── geometry_converter.rs       # IFC geometry → Mesh
│   │   ├── property_extractor.rs       # IFC properties → BIM properties
│   │   ├── schema/
│   │   │   ├── mod.rs
│   │   │   ├── ifc2x3.rs              # IFC 2x3 schema
│   │   │   ├── ifc4.rs                # IFC4 schema
│   │   │   └── ifc4x3.rs              # IFC 4.3 schema
│   │   └── entities/
│   │       ├── mod.rs
│   │       ├── spatial.rs             # Spatial structure converters
│   │       ├── products.rs            # Product element converters
│   │       └── geometry.rs            # Geometry converters
│   │
│   ├── gltf/                           # 📦 glTF Exporter Module
│   │   ├── mod.rs
│   │   ├── exporter.rs                # Main glTF/GLB exporter
│   │   ├── mesh_builder.rs            # Mesh buffer builder
│   │   ├── material_converter.rs      # IFC materials → PBR
│   │   └── scene_graph.rs             # Scene hierarchy builder
│   │
│   ├── db/                             # 📦 Database Module
│   │   ├── mod.rs
│   │   ├── models.rs                  # Database models
│   │   ├── queries.rs                 # Optimized SQL queries
│   │   └── repositories/
│   │       ├── mod.rs
│   │       ├── model_repo.rs          # Model repository
│   │       └── element_repo.rs        # Element repository
│   │
│   ├── spatial/                        # 📦 Spatial Analysis Module
│   │   ├── mod.rs
│   │   ├── bvh.rs                     # Bounding Volume Hierarchy
│   │   ├── octree.rs                  # Octree spatial indexing
│   │   ├── raycast.rs                 # Raycasting
│   │   ├── collision.rs               # Clash detection
│   │   └── visibility.rs              # Visibility analysis
│   │
│   ├── geometry/                       # 📦 Advanced Geometry Module
│   │   ├── mod.rs
│   │   ├── nurbs.rs                   # NURBS curves/surfaces
│   │   ├── tesselation.rs             # Surface → Mesh conversion
│   │   ├── boolean.rs                 # CSG boolean operations
│   │   └── brep/
│   │       ├── mod.rs
│   │       ├── topology.rs            # BRep topology structures
│   │       └── builder.rs             # BRep builder utilities
│   │
│   ├── cache/                          # 📦 Cache Module
│   │   ├── mod.rs
│   │   ├── geometry_cache.rs          # Geometry cache (Redis)
│   │   └── material_cache.rs          # Material cache
│   │
│   └── validation/                     # 📦 Validation Module
│       ├── mod.rs
│       ├── ifc_validator.rs           # IFC schema validator
│       ├── geometry_validator.rs      # Geometry quality validator
│       └── rules/
│           ├── mod.rs
│           ├── structural.rs          # Structural rules
│           └── clash_detection.rs     # Clash detection rules
│
└── migrations/                         # PostgreSQL migrations
    ├── 001_create_models.sql
    ├── 002_create_elements.sql
    ├── 003_create_properties.sql
    └── 004_create_projects.sql
```

## Módulos Implementados

### ✅ Core (Existente)
- `bim-core.rs` - Primitivos BIM
- `bim-converter.rs` - Worker de conversão

### 🆕 IFC Parser
- Parser STEP-File (ISO 10303-21)
- Schemas IFC 2x3, 4, 4.3
- Conversores de entidades espaciais e produtos
- Extração de geometria e propriedades

### 🆕 glTF Exporter
- Exportação GLB binária
- Construtor de scene graph
- Conversor de materiais PBR
- Merge de meshes otimizado

### 🆕 Database
- Models e repositories
- Queries otimizadas com índices
- Suporte a JSONB e arrays

### 🆕 Spatial Analysis
- BVH para aceleração espacial
- Octree 3D indexing
- Raycasting (Möller–Trumbore)
- Clash detection (hard/soft)
- Análise de visibilidade

### 🆕 Advanced Geometry
- NURBS curves/surfaces
- BRep topology completa
- Tesselation algorithms
- Boolean CSG operations

### 🆕 Cache
- Geometry cache com hash
- Material cache in-memory
- Preparado para Redis

### 🆕 Validation
- Validador IFC schema
- Validador de geometria (triângulos degenerados)
- Regras estruturais
- Regras de clash detection

## Total de Arquivos Criados
- **52 arquivos Rust** (.rs)
- **4 migrations SQL**
- **56 arquivos totais**

Estrutura completa implementada! 🚀
