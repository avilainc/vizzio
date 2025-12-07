# 🚀 avila-clustering - Complete Clustering Toolkit

State-of-the-art clustering algorithms for Rust, designed to surpass scikit-learn, HDBSCAN, and RAPIDS cuML in performance and capabilities.

## ✨ New Features (Expansion Pack)

### 📊 Complete Pipeline Support

```rust
use avila_clustering::prelude::*;
use avila_clustering::preprocessing::StandardScaler;
use avila_clustering::algorithms::kmeans::KMeans;
use avila_clustering::postprocessing::ClusterExplainer;
use avila_clustering::visualization::ProjectionEngine;

// 1. Preprocessing
let mut scaler = StandardScaler::new();
let data_scaled = scaler.fit_transform(&data.view())?;

// 2. Clustering
let mut kmeans = KMeans::new(3);
let labels = kmeans.fit(&data_scaled.view())?;

// 3. Post-processing
let explainer = ClusterExplainer::new();
let importance = explainer.feature_importance(&data_scaled.view(), &labels);

// 4. Visualization
let engine = ProjectionEngine::new(ProjectionType::PCA);
let projected = engine.project_2d(&data_scaled.view())?;
```

## 📦 Modules

### 🔧 Preprocessing (`preprocessing/`)
- **Normalization**: StandardScaler, MinMaxScaler, RobustScaler
- **Encoding**: LabelEncoder, OneHotEncoder, OrdinalEncoder
- **Feature Selection**: PCA, VarianceThreshold, FeatureSelector
- **Outlier Removal**: IQR, Z-Score, LocalOutlierFactor
- **Imputation**: SimpleImputer, KNNImputer

### 🎯 Algorithms (`algorithms/`)

#### Existing Algorithms
- **Partitional**: KMeans, KMedoids, Fuzzy C-Means, Mean Shift
- **Density-Based**: DBSCAN, HDBSCAN, OPTICS
- **Hierarchical**: Agglomerative, BIRCH
- **Model-Based**: GMM, Bayesian GMM
- **Graph-Based**: Spectral, Louvain, Leiden
- **Streaming**: Online clustering, Time-series

#### 🆕 Adaptive (`algorithms/adaptive/`)
- **AutoCluster**: Automatic k detection (Elbow, Silhouette, Gap Statistic, BIC)
- **ParameterTuner**: Grid search, Bayesian optimization
- **IncrementalClusterer**: Online/incremental learning
- **TransferClusterer**: Transfer learning between datasets

#### 🆕 Multimodal (`algorithms/multimodal/`)
- **KPrototypes**: Mixed numeric + categorical data
- **MultiViewClusterer**: Multiple feature spaces
- **TensorClusterer**: 3D+ tensor clustering
- **DynamicClusterer**: Time-evolving clusters

#### 🆕 Constrained (`algorithms/constrained/`)
- **SemiSupervisedClusterer**: Must-link/Cannot-link constraints
- **BalancedClusterer**: Size-balanced clusters
- **FairClusterer**: Fairness-aware clustering (demographic parity, etc.)
- **SpatialClusterer**: Geospatial constraints

### 🔍 Post-processing (`postprocessing/`)
- **Refinement**: Merge/split clusters, boundary adjustment
- **Labeling**: Auto-labeling, cluster naming
- **Explanation**: SHAP-like interpretability
- **Stability**: Temporal stability analysis

### 🎨 Visualization (`visualization/`)
- **Projection**: 2D/3D projections (PCA, t-SNE, Random)
- **Dendrogram**: Hierarchical clustering trees
- **GraphLayout**: Force-directed, circular, spring layouts
- **Export**: JSON, CSV, GeoJSON for D3.js, Plotly, Leaflet

### 📈 Metrics (`metrics/`)
- **Distance**: Euclidean, Manhattan, Cosine, etc.
- **Validation**: Silhouette, Calinski-Harabasz, Davies-Bouldin
- **Interpretability**: Feature importance, cluster profiles, separation analysis

### 💾 I/O (`io/`)
- **Serialization**: Save/load models (JSON, binary)
- **Formats**: CSV, Parquet, Arrow import
- **Streaming**: Chunked reading for big data
- **Cache**: Intelligent result caching

### 🛠️ Utils (`utils/`)
- **Sampling**: Strategic sampling, stratified
- **Random**: Seedable RNG
- **Parallel**: Parallelization helpers
- **Memory**: Memory management for large datasets

### 🧪 Scientific (`scientific/`)
- **Physics**: Physics-based clustering
- **Spacetime**: 4D spacetime clustering
- **Curved**: Curved manifolds
- **Quantum**: QAOA, VQE, Quantum Annealing (classical simulation)

### 🎮 Integration (`integration/`)
- **Pipeline**: Composable sklearn-like pipelines
- **CrossValidation**: Clustering-specific CV
- **EnsembleMeta**: Meta-ensemble of algorithms
- **AutoML**: Automatic algorithm selection

### ⚡ Benchmarks (`benchmarks/`)
- **Performance**: Time/memory tracking
- **Quality**: Aggregated quality metrics
- **Scalability**: Scaling tests
- **Comparison**: Algorithm comparison tools

## 🚀 Examples

### Complete Pipeline
```bash
cargo run --example complete_pipeline
```

### Adaptive Algorithms
```bash
cargo run --example adaptive_algorithms
```

### Constrained Clustering
```bash
cargo run --example constrained_clustering
```

### Visualization & Export
```bash
cargo run --example visualization_export
```

## 📊 Performance

- **Zero external dependencies** (pure Rust)
- **GPU acceleration** (CUDA, ROCm, Vulkan, Metal, OpenCL)
- **Parallel processing** built-in
- **Memory efficient** for big data
- **Streaming support** for infinite data

## 🎯 Use Cases

### Business Analytics
- Customer segmentation with fairness constraints
- Market basket analysis with balanced clusters
- Geospatial customer zoning

### Machine Learning
- Feature engineering with auto-selection
- Semi-supervised learning with partial labels
- Transfer learning across domains

### Scientific Computing
- Quantum chemistry simulations
- Astronomical data clustering
- Physics-based particle grouping

### Data Visualization
- Interactive D3.js dashboards
- Geographic heatmaps (GeoJSON)
- Hierarchical dendrograms

## 🏗️ Architecture

```
src/
├── preprocessing/      # Data preparation
├── algorithms/         # Clustering algorithms
│   ├── adaptive/      # Self-tuning algorithms
│   ├── multimodal/    # Multi-type data
│   └── constrained/   # Constraint-based
├── postprocessing/    # Result refinement
├── visualization/     # Export for viz
├── metrics/           # Validation & interpretation
├── io/                # Serialization & formats
├── utils/             # Internal utilities
├── scientific/        # Advanced scientific
├── integration/       # Pipelines & AutoML
├── benchmarks/        # Performance tracking
└── gpu/               # GPU backends
```

## 📚 Documentation

Full API documentation:
```bash
cargo doc --open
```

## 🤝 Contributing

Contributions welcome! This is a modular, extensible architecture designed for:
- Easy addition of new algorithms
- Zero external dependencies policy
- Performance-first implementation
- Scientific rigor

## 📄 License

[Your License Here]

## 🎉 What's New

**Version 2.0 - Complete Expansion**
- ✅ 8 new major modules
- ✅ 7 new algorithm categories
- ✅ 50+ new functions
- ✅ Complete preprocessing pipeline
- ✅ Advanced post-processing
- ✅ Multi-format visualization export
- ✅ Adaptive & constrained algorithms
- ✅ Fairness-aware clustering
- ✅ Quantum clustering (simulation)
- ✅ AutoML for clustering

---

Built with ❤️ in Rust for maximum performance and safety.
