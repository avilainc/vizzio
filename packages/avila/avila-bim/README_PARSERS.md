# Infraestrutura de Parsers de Arquivos CAD/BIM

Esta biblioteca fornece uma infraestrutura completa para leitura e processamento de arquivos de formatos CAD/BIM utilizados na construção civil e engenharia.

## Formatos Suportados

### Formatos Implementados

- **IFC (Industry Foundation Classes)**: Formato padrão BIM com parsing completo de entidades STEP
- **DWG (AutoCAD Drawing)**: Formato binário nativo do AutoCAD (detecção básica e metadados)
- **DXF (Drawing Exchange Format)**: Formato ASCII do AutoCAD com suporte a entidades 2D
- **OBJ (Wavefront OBJ)**: Formato de malha 3D com triangulação automática
- **STL (STereoLithography)**: Formato de malha 3D (ASCII e binário)
- **PLY (Polygon File Format)**: Formato de nuvem de pontos e malhas (ASCII)

### Formatos com Stubs (Não Implementados)

- **FBX**: Formato de intercâmbio da Autodesk
- **glTF/GLB**: Formato 3D otimizado para web
- **SKP**: Formato do SketchUp
- **RVT**: Formato do Revit
- **NWD**: Formato do Navisworks

## Arquitetura

### Estruturas de Dados Principais

```rust
pub struct LoadedModel {
    pub elements: Vec<ModelElement>,
    pub materials: Vec<Material>,
    pub metadata: HashMap<String, String>,
}

pub struct ModelElement {
    pub name: String,
    pub element_type: ElementType,
    pub geometry: Option<ElementGeometry>,
    pub properties: HashMap<String, PropertyValue>,
}

pub struct ElementGeometry {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub primitive_type: PrimitiveType,
}
```

### Trait FileParser

Todos os parsers implementam o trait `FileParser`:

```rust
pub trait FileParser {
    fn can_parse(&self, format: FileFormat) -> bool;
    fn parse(&self, data: &[u8]) -> Result<LoadedModel, ParseError>;
}
```

### Gerenciador de Parsers

O `ParserManager` gerencia todos os parsers registrados:

```rust
let mut parser_manager = ParserManager::new();
parser_manager.register_parser(FileFormat::IFC, Box::new(IfcFileParser::new()));

let model = parser_manager.parse_file("arquivo.ifc")?;
```

## Uso Básico

```rust
use avila_bim::{ParserManager, FileFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser_manager = ParserManager::new();

    // Registrar parsers
    parser_manager.register_parser(FileFormat::IFC, Box::new(IfcFileParser::new()));
    parser_manager.register_parser(FileFormat::DXF, Box::new(DxfFileParser::new()));

    // Carregar arquivo
    let model = parser_manager.parse_file("projeto.ifc")?;

    println!("Carregado {} elementos", model.elements.len());

    Ok(())
}
```

## Detalhes dos Parsers

### IFC Parser

- Parsing completo do formato STEP Physical File
- Extração de entidades IFC (walls, slabs, beams, etc.)
- Conversão de geometria para malhas trianguladas
- Suporte a propriedades e metadados

### DWG Parser

- Detecção de arquivos DWG válidos por assinatura
- Suporte a múltiplas versões (R13, R14, R2000, R2004, R2007, R2010, R2013, R2018)
- Extração de metadados básicos (versão, tamanho do arquivo)
- **Nota**: Parsing completo de geometria DWG é muito complexo devido ao formato proprietário. Atualmente retorna apenas metadados.

### DXF Parser

- Parsing de seções HEADER, TABLES, BLOCKS, ENTITIES
- Suporte a entidades: LINE, CIRCLE, ARC, LWPOLYLINE, TEXT
- Conversão para geometria unificada

### OBJ Parser

- Parsing de vértices, normais e coordenadas UV
- Triangulação automática de faces
- Suporte a grupos e materiais

### STL Parser

- Suporte a formato ASCII e binário
- Validação de geometria
- Cálculo automático de normais se necessário

### PLY Parser

- Parsing de header com propriedades
- Suporte a dados ASCII
- Extração de geometria e propriedades customizadas

## Extensibilidade

Para adicionar um novo formato:

1. Implementar o trait `FileParser`
2. Adicionar o formato ao enum `FileFormat`
3. Registrar no `ParserManager`

## Tratamento de Erros

Todos os parsers retornam `Result<LoadedModel, ParseError>` com erros específicos:

- `IoError`: Erros de I/O
- `ParseError`: Erros de parsing
- `UnsupportedFormat`: Formato não suportado
- `InvalidData`: Dados inválidos

## Próximos Passos

- **Completar parser DWG**: Implementar extração de geometria das seções comprimidas
- Completar parsers de formatos proprietários (FBX, glTF, SKP, RVT, NWD)
- Adicionar suporte a materiais e texturas
- Implementar validação de geometria
- Adicionar suporte a animações e esqueletos
