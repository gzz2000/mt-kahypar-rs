/* automatically generated by rust-bindgen 0.72.0 */

pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    const UNINIT: ::std::mem::MaybeUninit<max_align_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        "Size of max_align_t"
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        "Alignment of max_align_t"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce1) as usize - ptr as usize },
        0usize,
        "Offset of field: max_align_t::__clang_max_align_nonce1"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce2) as usize - ptr as usize },
        16usize,
        "Offset of field: max_align_t::__clang_max_align_nonce2"
    );
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mt_kahypar_hypergraph_type_t {
    STATIC_GRAPH = 0,
    DYNAMIC_GRAPH = 1,
    STATIC_HYPERGRAPH = 2,
    DYNAMIC_HYPERGRAPH = 3,
    NULLPTR_HYPERGRAPH = 4,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mt_kahypar_partition_type_t {
    MULTILEVEL_GRAPH_PARTITIONING = 0,
    N_LEVEL_GRAPH_PARTITIONING = 1,
    MULTILEVEL_HYPERGRAPH_PARTITIONING = 2,
    N_LEVEL_HYPERGRAPH_PARTITIONING = 3,
    LARGE_K_PARTITIONING = 4,
    NULLPTR_PARTITION = 5,
}
#[repr(u32)]
#[non_exhaustive]
#[doc = " Either success or type of the error."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mt_kahypar_status_t {
    SUCCESS = 0,
    INVALID_INPUT = 1,
    INVALID_PARAMETER = 2,
    UNSUPPORTED_OPERATION = 3,
    SYSTEM_ERROR = 4,
    OTHER_ERROR = 5,
}
#[doc = " Indicates whether an error occured.\n\n If an error occurs, mt_kahypar_free_error_content needs to be called\n afterwards to free the space allocated for the error message."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_error_t {
    pub msg: *const ::std::os::raw::c_char,
    pub msg_len: usize,
    pub status: mt_kahypar_status_t,
}
#[test]
fn bindgen_test_layout_mt_kahypar_error_t() {
    const UNINIT: ::std::mem::MaybeUninit<mt_kahypar_error_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mt_kahypar_error_t>(),
        24usize,
        "Size of mt_kahypar_error_t"
    );
    assert_eq!(
        ::std::mem::align_of::<mt_kahypar_error_t>(),
        8usize,
        "Alignment of mt_kahypar_error_t"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).msg) as usize - ptr as usize },
        0usize,
        "Offset of field: mt_kahypar_error_t::msg"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).msg_len) as usize - ptr as usize },
        8usize,
        "Offset of field: mt_kahypar_error_t::msg_len"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).status) as usize - ptr as usize },
        16usize,
        "Offset of field: mt_kahypar_error_t::status"
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_context_s {
    _unused: [u8; 0],
}
pub type mt_kahypar_context_t = mt_kahypar_context_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_target_graph_s {
    _unused: [u8; 0],
}
pub type mt_kahypar_target_graph_t = mt_kahypar_target_graph_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_hypergraph_s {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_hypergraph_t {
    pub hypergraph: *mut mt_kahypar_hypergraph_s,
    pub type_: mt_kahypar_hypergraph_type_t,
}
#[test]
fn bindgen_test_layout_mt_kahypar_hypergraph_t() {
    const UNINIT: ::std::mem::MaybeUninit<mt_kahypar_hypergraph_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mt_kahypar_hypergraph_t>(),
        16usize,
        "Size of mt_kahypar_hypergraph_t"
    );
    assert_eq!(
        ::std::mem::align_of::<mt_kahypar_hypergraph_t>(),
        8usize,
        "Alignment of mt_kahypar_hypergraph_t"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hypergraph) as usize - ptr as usize },
        0usize,
        "Offset of field: mt_kahypar_hypergraph_t::hypergraph"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        "Offset of field: mt_kahypar_hypergraph_t::type_"
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_hypergraph_const_t {
    pub hypergraph: *const mt_kahypar_hypergraph_s,
    pub type_: mt_kahypar_hypergraph_type_t,
}
#[test]
fn bindgen_test_layout_mt_kahypar_hypergraph_const_t() {
    const UNINIT: ::std::mem::MaybeUninit<mt_kahypar_hypergraph_const_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mt_kahypar_hypergraph_const_t>(),
        16usize,
        "Size of mt_kahypar_hypergraph_const_t"
    );
    assert_eq!(
        ::std::mem::align_of::<mt_kahypar_hypergraph_const_t>(),
        8usize,
        "Alignment of mt_kahypar_hypergraph_const_t"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).hypergraph) as usize - ptr as usize },
        0usize,
        "Offset of field: mt_kahypar_hypergraph_const_t::hypergraph"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        "Offset of field: mt_kahypar_hypergraph_const_t::type_"
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_partitioned_hypergraph_s {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_partitioned_hypergraph_t {
    pub partitioned_hg: *mut mt_kahypar_partitioned_hypergraph_s,
    pub type_: mt_kahypar_partition_type_t,
}
#[test]
fn bindgen_test_layout_mt_kahypar_partitioned_hypergraph_t() {
    const UNINIT: ::std::mem::MaybeUninit<mt_kahypar_partitioned_hypergraph_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mt_kahypar_partitioned_hypergraph_t>(),
        16usize,
        "Size of mt_kahypar_partitioned_hypergraph_t"
    );
    assert_eq!(
        ::std::mem::align_of::<mt_kahypar_partitioned_hypergraph_t>(),
        8usize,
        "Alignment of mt_kahypar_partitioned_hypergraph_t"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).partitioned_hg) as usize - ptr as usize },
        0usize,
        "Offset of field: mt_kahypar_partitioned_hypergraph_t::partitioned_hg"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        "Offset of field: mt_kahypar_partitioned_hypergraph_t::type_"
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mt_kahypar_partitioned_hypergraph_const_t {
    pub partitioned_hg: *const mt_kahypar_partitioned_hypergraph_s,
    pub type_: mt_kahypar_partition_type_t,
}
#[test]
fn bindgen_test_layout_mt_kahypar_partitioned_hypergraph_const_t() {
    const UNINIT: ::std::mem::MaybeUninit<mt_kahypar_partitioned_hypergraph_const_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mt_kahypar_partitioned_hypergraph_const_t>(),
        16usize,
        "Size of mt_kahypar_partitioned_hypergraph_const_t"
    );
    assert_eq!(
        ::std::mem::align_of::<mt_kahypar_partitioned_hypergraph_const_t>(),
        8usize,
        "Alignment of mt_kahypar_partitioned_hypergraph_const_t"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).partitioned_hg) as usize - ptr as usize },
        0usize,
        "Offset of field: mt_kahypar_partitioned_hypergraph_const_t::partitioned_hg"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        8usize,
        "Offset of field: mt_kahypar_partitioned_hypergraph_const_t::type_"
    );
}
pub type mt_kahypar_hypernode_id_t = ::std::os::raw::c_ulong;
pub type mt_kahypar_hyperedge_id_t = ::std::os::raw::c_ulong;
pub type mt_kahypar_hypernode_weight_t = ::std::os::raw::c_int;
pub type mt_kahypar_hyperedge_weight_t = ::std::os::raw::c_int;
pub type mt_kahypar_partition_id_t = ::std::os::raw::c_int;
#[repr(u32)]
#[non_exhaustive]
#[doc = " Configurable parameters of the partitioning context."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mt_kahypar_context_parameter_type_t {
    NUM_BLOCKS = 0,
    EPSILON = 1,
    OBJECTIVE = 2,
    NUM_VCYCLES = 3,
    VERBOSE = 4,
}
#[repr(u32)]
#[non_exhaustive]
#[doc = " Supported objective functions."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mt_kahypar_objective_t {
    CUT = 0,
    KM1 = 1,
    SOED = 2,
}
#[repr(u32)]
#[non_exhaustive]
#[doc = " Preset types for partitioning context."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mt_kahypar_preset_type_t {
    DETERMINISTIC = 0,
    LARGE_K = 1,
    DEFAULT = 2,
    QUALITY = 3,
    HIGHEST_QUALITY = 4,
}
#[repr(u32)]
#[non_exhaustive]
#[doc = " Supported (hyper)graph file formats."]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mt_kahypar_file_format_type_t {
    METIS = 0,
    HMETIS = 1,
}
extern "C" {
    #[doc = " Deletes the partitioning context object."]
    pub fn mt_kahypar_free_context(context: *mut mt_kahypar_context_t);
}
extern "C" {
    #[doc = " Loads a partitioning context from a configuration file."]
    pub fn mt_kahypar_context_from_file(
        ini_file_name: *const ::std::os::raw::c_char,
        error: *mut mt_kahypar_error_t,
    ) -> *mut mt_kahypar_context_t;
}
extern "C" {
    #[doc = " Loads a partitioning context of a predefined preset type.\n\n See 'mt_kahypar_preset_type_t' for possible presets."]
    pub fn mt_kahypar_context_from_preset(
        preset: mt_kahypar_preset_type_t,
    ) -> *mut mt_kahypar_context_t;
}
extern "C" {
    #[doc = " Sets a new value for a context parameter.\n\n Usage:\n mt_kahypar_set_context_parameter(context, OBJECTIVE, \"km1\", &error) // sets the objective function to the connectivity metric\n\n \\return success (zero) if the corresponding parameter is successfully set to the value. Otherwise,\n returns INVALID_PARAMETER and error is set accordingly."]
    pub fn mt_kahypar_set_context_parameter(
        context: *mut mt_kahypar_context_t,
        type_: mt_kahypar_context_parameter_type_t,
        value: *const ::std::os::raw::c_char,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_status_t;
}
extern "C" {
    #[doc = " Sets all required parameters for a partitioning call."]
    pub fn mt_kahypar_set_partitioning_parameters(
        context: *mut mt_kahypar_context_t,
        num_blocks: mt_kahypar_partition_id_t,
        epsilon: f64,
        objective: mt_kahypar_objective_t,
    );
}
extern "C" {
    pub fn mt_kahypar_get_preset(context: *const mt_kahypar_context_t) -> mt_kahypar_preset_type_t;
}
extern "C" {
    #[doc = " Get number of blocks. Result is unspecified if not previously initialized."]
    pub fn mt_kahypar_get_num_blocks(
        context: *const mt_kahypar_context_t,
    ) -> mt_kahypar_partition_id_t;
}
extern "C" {
    #[doc = " Get imbalance parameter epsilon. Result is unspecified if not previously initialized."]
    pub fn mt_kahypar_get_epsilon(context: *const mt_kahypar_context_t) -> f64;
}
extern "C" {
    #[doc = " Get objective function. Result is unspecified if not previously initialized."]
    pub fn mt_kahypar_get_objective(context: *const mt_kahypar_context_t)
        -> mt_kahypar_objective_t;
}
extern "C" {
    #[doc = " Initializes the random number generator with the given seed value (not thread-safe)."]
    pub fn mt_kahypar_set_seed(seed: usize);
}
extern "C" {
    #[doc = " Sets individual target block weights for each block of the partition.\n A balanced partition then satisfies that the weight of each block is smaller or equal than the\n defined target block weight for the corresponding block.\n\n Note that the context is invalid if the number of block weights is not equal to the NUM_BLOCKS parameter."]
    pub fn mt_kahypar_set_individual_target_block_weights(
        context: *mut mt_kahypar_context_t,
        num_blocks: mt_kahypar_partition_id_t,
        block_weights: *const mt_kahypar_hypernode_weight_t,
    );
}
extern "C" {
    #[doc = " Must be called once for global initialization, before trying to create or partition any (hyper)graph.\n\n Note: if 'num_threads' is larger than the number of actually available CPUs, only a reduced number of threads will be used."]
    pub fn mt_kahypar_initialize(num_threads: usize, interleaved_allocations: bool);
}
extern "C" {
    #[doc = " Frees the content of the error."]
    pub fn mt_kahypar_free_error_content(error: *mut mt_kahypar_error_t);
}
extern "C" {
    #[doc = " Reads a (hyper)graph from a file for a given configuration (preset).\n The file can be either in hMetis or Metis file format.\n\n \\note Note that we use different (hyper)graph data structures for different configurations.\n Make sure that you partition the hypergraph with the same configuration as it is loaded."]
    pub fn mt_kahypar_read_hypergraph_from_file(
        file_name: *const ::std::os::raw::c_char,
        context: *const mt_kahypar_context_t,
        file_format: mt_kahypar_file_format_type_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_hypergraph_t;
}
extern "C" {
    #[doc = " Reads a target graph in Metis file format. The target graph can be used in the\n 'mt_kahypar_map' function to map a (hyper)graph onto it."]
    pub fn mt_kahypar_read_target_graph_from_file(
        file_name: *const ::std::os::raw::c_char,
        context: *const mt_kahypar_context_t,
        error: *mut mt_kahypar_error_t,
    ) -> *mut mt_kahypar_target_graph_t;
}
extern "C" {
    #[doc = " Constructs a hypergraph from a given adjacency array that specifies the hyperedges.\n\n For example:\n hyperedge_indices: | 0   | 2       | 6     | 9     | 12\n hyperedges:        | 0 2 | 0 1 3 4 | 3 4 6 | 2 5 6 |\n Defines a hypergraph with four hyperedges, e.g., e_0 = {0,2}, e_1 = {0,1,3,4}, ...\n\n \\note For unweighted hypergraphs, you can pass nullptr to either hyperedge_weights or vertex_weights.\n \\note After construction, the arguments of this function are no longer needed and can be deleted."]
    pub fn mt_kahypar_create_hypergraph(
        context: *const mt_kahypar_context_t,
        num_vertices: mt_kahypar_hypernode_id_t,
        num_hyperedges: mt_kahypar_hyperedge_id_t,
        hyperedge_indices: *const usize,
        hyperedges: *const mt_kahypar_hyperedge_id_t,
        hyperedge_weights: *const mt_kahypar_hyperedge_weight_t,
        vertex_weights: *const mt_kahypar_hypernode_weight_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_hypergraph_t;
}
extern "C" {
    #[doc = " Constructs a graph from a given edge list vector.\n\n Example:\n edges:        | 0 2 | 0 1 | 2 3 | 1 3 |\n Defines a graph with four edges -> e_0 = {0,2}, e_1 = {0,1}, e_2 = {2,3}, e_3 = {1,3}\n\n \\note For unweighted graphs, you can pass nullptr to either hyperedge_weights or vertex_weights.\n \\note After construction, the arguments of this function are no longer needed and can be deleted."]
    pub fn mt_kahypar_create_graph(
        context: *const mt_kahypar_context_t,
        num_vertices: mt_kahypar_hypernode_id_t,
        num_edges: mt_kahypar_hyperedge_id_t,
        edges: *const mt_kahypar_hypernode_id_t,
        edge_weights: *const mt_kahypar_hyperedge_weight_t,
        vertex_weights: *const mt_kahypar_hypernode_weight_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_hypergraph_t;
}
extern "C" {
    #[doc = " Constructs a target graph from a given edge list vector. The target graph can be used in the\n 'mt_kahypar_map' function to map a (hyper)graph onto it.\n\n Example:\n edges:        | 0 2 | 0 1 | 2 3 | 1 3 |\n Defines a graph with four edges -> e_0 = {0,2}, e_1 = {0,1}, e_2 = {2,3}, e_3 = {1,3}\n\n \\note For unweighted graphs, you can pass nullptr to either hyperedge_weights.\n \\note After construction, the arguments of this function are no longer needed and can be deleted."]
    pub fn mt_kahypar_create_target_graph(
        context: *const mt_kahypar_context_t,
        num_vertices: mt_kahypar_hypernode_id_t,
        num_edges: mt_kahypar_hyperedge_id_t,
        edges: *const mt_kahypar_hypernode_id_t,
        edge_weights: *const mt_kahypar_hyperedge_weight_t,
        error: *mut mt_kahypar_error_t,
    ) -> *mut mt_kahypar_target_graph_t;
}
extern "C" {
    #[doc = " Deletes the (hyper)graph object."]
    pub fn mt_kahypar_free_hypergraph(hypergraph: mt_kahypar_hypergraph_t);
}
extern "C" {
    #[doc = " Deletes a target graph object."]
    pub fn mt_kahypar_free_target_graph(target_graph: *mut mt_kahypar_target_graph_t);
}
extern "C" {
    #[doc = " Returns the number of nodes of the (hyper)graph."]
    pub fn mt_kahypar_num_hypernodes(
        hypergraph: mt_kahypar_hypergraph_t,
    ) -> mt_kahypar_hypernode_id_t;
}
extern "C" {
    #[doc = " Returns the number of (hyper)edges of the (hyper)graph.\n\n Note that for graphs, this returns the number of directed edges (i.e., twice the number of undirected edges)."]
    pub fn mt_kahypar_num_hyperedges(
        hypergraph: mt_kahypar_hypergraph_t,
    ) -> mt_kahypar_hyperedge_id_t;
}
extern "C" {
    #[doc = " Returns the number of pins of the hypergraph."]
    pub fn mt_kahypar_num_pins(hypergraph: mt_kahypar_hypergraph_t) -> mt_kahypar_hypernode_id_t;
}
extern "C" {
    #[doc = " Returns the sum of all node weights of the (hyper)graph."]
    pub fn mt_kahypar_hypergraph_weight(
        hypergraph: mt_kahypar_hypergraph_t,
    ) -> mt_kahypar_hypernode_id_t;
}
extern "C" {
    #[doc = " Returns the degree of the corresponding node."]
    pub fn mt_kahypar_hypernode_degree(
        hypergraph: mt_kahypar_hypergraph_t,
        node: mt_kahypar_hypernode_id_t,
    ) -> mt_kahypar_hyperedge_id_t;
}
extern "C" {
    #[doc = " Returns the weight of the corresponding node."]
    pub fn mt_kahypar_hypernode_weight(
        hypergraph: mt_kahypar_hypergraph_t,
        node: mt_kahypar_hypernode_id_t,
    ) -> mt_kahypar_hypernode_weight_t;
}
extern "C" {
    #[doc = " Returns the size of the corresponding hyperedge.\n\n Note that for graphs, the size is always 2."]
    pub fn mt_kahypar_hyperedge_size(
        hypergraph: mt_kahypar_hypergraph_t,
        edge: mt_kahypar_hyperedge_id_t,
    ) -> mt_kahypar_hypernode_id_t;
}
extern "C" {
    #[doc = " Returns the weight of the corresponding edge."]
    pub fn mt_kahypar_hyperedge_weight(
        hypergraph: mt_kahypar_hypergraph_t,
        edge: mt_kahypar_hyperedge_id_t,
    ) -> mt_kahypar_hyperedge_weight_t;
}
extern "C" {
    #[doc = " Writes the IDs of the hyperedges that are incident to the corresponding node into the provided buffer.\n The size of the provided array must be at least 'mt_kahypar_hypernode_degree(node)'\n (note that 'mt_kahypar_num_hyperedges' also provides an upper bound).\n\n \\return the number of returned hyperedges"]
    pub fn mt_kahypar_get_incident_hyperedges(
        hypergraph: mt_kahypar_hypergraph_t,
        node: mt_kahypar_hypernode_id_t,
        edge_buffer: *mut mt_kahypar_hyperedge_id_t,
    ) -> mt_kahypar_hyperedge_id_t;
}
extern "C" {
    #[doc = " Writes the IDs of the pins (i.e., hypernodes) in the corresponding hyperedge into the provided buffer.\n The size of the provided array must be at least 'mt_kahypar_hyperedge_size(edge)'\n (note that 'mt_kahypar_num_hypernodes' also provides an upper bound).\n\n \\return the number of returned pins"]
    pub fn mt_kahypar_get_hyperedge_pins(
        hypergraph: mt_kahypar_hypergraph_t,
        edge: mt_kahypar_hyperedge_id_t,
        pin_buffer: *mut mt_kahypar_hypernode_id_t,
    ) -> mt_kahypar_hypernode_id_t;
}
extern "C" {
    #[doc = " Returns whether 'hypergraph' is a graph (i.e., not a hypergraph)."]
    pub fn mt_kahypar_is_graph(hypergraph: mt_kahypar_hypergraph_t) -> bool;
}
extern "C" {
    #[doc = " Source of the corresponding graph edge.\n Returns 0 if 'graph' is not a graph but a hypergraph."]
    pub fn mt_kahypar_edge_source(
        graph: mt_kahypar_hypergraph_t,
        edge: mt_kahypar_hyperedge_id_t,
    ) -> mt_kahypar_hypernode_id_t;
}
extern "C" {
    #[doc = " Target of the corresponding graph edge.\n Returns 0 if 'graph' is not a graph but a hypergraph."]
    pub fn mt_kahypar_edge_target(
        graph: mt_kahypar_hypergraph_t,
        edge: mt_kahypar_hyperedge_id_t,
    ) -> mt_kahypar_hypernode_id_t;
}
extern "C" {
    #[doc = " Adds fixed vertices to the (hyper)graph as specified in the array to which 'fixed_vertices' points to.\n The array should contain n entries (n = number of nodes). Each entry contains either the fixed vertex\n block ID of the corresponding node or -1 if the node is not fixed to a block."]
    pub fn mt_kahypar_add_fixed_vertices(
        hypergraph: mt_kahypar_hypergraph_t,
        fixed_vertices: *const mt_kahypar_partition_id_t,
        num_blocks: mt_kahypar_partition_id_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_status_t;
}
extern "C" {
    #[doc = " Reads fixed vertices from a file and stores them in the array to which 'fixed_vertices' points to.\n The array must be of size 'num_nodes'. If the number of entries in the file is different from 'num_nodes',\n an error is returned."]
    pub fn mt_kahypar_read_fixed_vertices_from_file(
        file_name: *const ::std::os::raw::c_char,
        num_nodes: mt_kahypar_hypernode_id_t,
        fixed_vertices: *mut mt_kahypar_partition_id_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_status_t;
}
extern "C" {
    #[doc = " Adds fixed vertices to the (hyper)graph as specified in the fixed vertex file (expected in hMetis fix file format).\n The file should contain n lines (n = number of nodes). Each line contains either the fixed vertex\n block ID of the corresponding node or -1 if the node is not fixed to a block."]
    pub fn mt_kahypar_add_fixed_vertices_from_file(
        hypergraph: mt_kahypar_hypergraph_t,
        file_name: *const ::std::os::raw::c_char,
        num_blocks: mt_kahypar_partition_id_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_status_t;
}
extern "C" {
    #[doc = " Removes all fixed vertices from the hypergraph."]
    pub fn mt_kahypar_remove_fixed_vertices(hypergraph: mt_kahypar_hypergraph_t);
}
extern "C" {
    #[doc = " Whether the corresponding node is a fixed vertex."]
    pub fn mt_kahypar_is_fixed_vertex(
        hypergraph: mt_kahypar_hypergraph_t,
        node: mt_kahypar_hypernode_id_t,
    ) -> bool;
}
extern "C" {
    #[doc = " Block to which the node is fixed (-1 if not fixed)."]
    pub fn mt_kahypar_fixed_vertex_block(
        hypergraph: mt_kahypar_hypergraph_t,
        node: mt_kahypar_hypernode_id_t,
    ) -> mt_kahypar_partition_id_t;
}
extern "C" {
    #[doc = " Checks whether or not the given hypergraph can be partitioned with the corresponding preset."]
    pub fn mt_kahypar_check_compatibility(
        hypergraph: mt_kahypar_hypergraph_t,
        preset: mt_kahypar_preset_type_t,
    ) -> bool;
}
extern "C" {
    #[doc = " Partitions a (hyper)graph with the configuration specified in the partitioning context.\n\n \\note Before partitioning, the number of blocks, imbalance parameter and objective function must be\n       set in the partitioning context. This can be done either via mt_kahypar_set_context_parameter(...)\n       or mt_kahypar_set_partitioning_parameters(...)."]
    pub fn mt_kahypar_partition(
        hypergraph: mt_kahypar_hypergraph_t,
        context: *const mt_kahypar_context_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_partitioned_hypergraph_t;
}
extern "C" {
    #[doc = " Maps a (hyper)graph onto a target graph with the configuration specified in the partitioning context.\n The number of blocks of the output mapping/partition is the same as the number of nodes in the target graph\n (each node of the target graph represents a block). The objective is to minimize the total weight of\n all Steiner trees spanned by the (hyper)edges on the target graph. A Steiner tree is a tree with minimal weight\n that spans a subset of the nodes (in our case the hyperedges) on the target graph. This objective function\n is able to acurately model wire-lengths in VLSI design or communication costs in a distributed system where some\n processors do not communicate directly with each other or different speeds.\n\n \\note Since computing Steiner trees is an NP-hard problem, we currently restrict the size of the target graph\n to at most 64 nodes. If you want to map hypergraphs onto larger target graphs, you can use recursive multisectioning.\n For example, if the target graph has 4096 nodes, you can first map the hypergraph onto a coarser approximation of the\n target graph with 64 nodes, and subsequently map each block of the mapping to the corresponding subgraph of the\n target graph each having 64 nodes."]
    pub fn mt_kahypar_map(
        hypergraph: mt_kahypar_hypergraph_t,
        target_graph: *mut mt_kahypar_target_graph_t,
        context: *const mt_kahypar_context_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_partitioned_hypergraph_t;
}
extern "C" {
    #[doc = " Checks whether or not the given partitioned hypergraph can\n be improved with the corresponding preset."]
    pub fn mt_kahypar_check_partition_compatibility(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        preset: mt_kahypar_preset_type_t,
    ) -> bool;
}
extern "C" {
    #[doc = " Improves a given partition (using the V-cycle technique).\n\n \\note The number of blocks specified in the partitioning context must be equal to the\n       number of blocks of the given partition.\n \\note There is no guarantee that this call will find an improvement."]
    pub fn mt_kahypar_improve_partition(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        context: *const mt_kahypar_context_t,
        num_vcycles: usize,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_status_t;
}
extern "C" {
    #[doc = " Improves a given mapping (using the V-cycle technique).\n\n \\note The number of nodes of the target graph must be equal to the\n       number of blocks of the given partition.\n \\note There is no guarantee that this call will find an improvement."]
    pub fn mt_kahypar_improve_mapping(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        target_graph: *mut mt_kahypar_target_graph_t,
        context: *const mt_kahypar_context_t,
        num_vcycles: usize,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_status_t;
}
extern "C" {
    #[doc = " Constructs a partitioned (hyper)graph out of the given partition."]
    pub fn mt_kahypar_create_partitioned_hypergraph(
        hypergraph: mt_kahypar_hypergraph_t,
        context: *const mt_kahypar_context_t,
        num_blocks: mt_kahypar_partition_id_t,
        partition: *const mt_kahypar_partition_id_t,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_partitioned_hypergraph_t;
}
extern "C" {
    #[doc = " Constructs a partitioned (hyper)graph from a given partition file."]
    pub fn mt_kahypar_read_partition_from_file(
        hypergraph: mt_kahypar_hypergraph_t,
        context: *const mt_kahypar_context_t,
        num_blocks: mt_kahypar_partition_id_t,
        partition_file: *const ::std::os::raw::c_char,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_partitioned_hypergraph_t;
}
extern "C" {
    #[doc = " Writes a partition to a file."]
    pub fn mt_kahypar_write_partition_to_file(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        partition_file: *const ::std::os::raw::c_char,
        error: *mut mt_kahypar_error_t,
    ) -> mt_kahypar_status_t;
}
extern "C" {
    #[doc = " Number of blocks of the partition."]
    pub fn mt_kahypar_num_blocks(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
    ) -> mt_kahypar_partition_id_t;
}
extern "C" {
    #[doc = " Weight of the corresponding block."]
    pub fn mt_kahypar_block_weight(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        block: mt_kahypar_partition_id_t,
    ) -> mt_kahypar_hypernode_weight_t;
}
extern "C" {
    #[doc = " Block to which the corresponding hypernode is assigned."]
    pub fn mt_kahypar_block_id(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        node: mt_kahypar_hypernode_id_t,
    ) -> mt_kahypar_partition_id_t;
}
extern "C" {
    #[doc = " Number of incident cut hyperedges of the corresponding node."]
    pub fn mt_kahypar_num_incident_cut_hyperedges(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        node: mt_kahypar_hypernode_id_t,
    ) -> mt_kahypar_hyperedge_id_t;
}
extern "C" {
    #[doc = " Number of distinct blocks to which the pins of the corresponding hyperedge are assigned."]
    pub fn mt_kahypar_connectivity(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        edge: mt_kahypar_hyperedge_id_t,
    ) -> mt_kahypar_partition_id_t;
}
extern "C" {
    #[doc = " Number of pins assigned to the corresponding block in the given hyperedge."]
    pub fn mt_kahypar_num_pins_in_block(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        edge: mt_kahypar_hyperedge_id_t,
        block: mt_kahypar_partition_id_t,
    ) -> mt_kahypar_hypernode_id_t;
}
extern "C" {
    #[doc = " Extracts a partition from a partitioned (hyper)graph. The size of the provided array must be at least the number of nodes."]
    pub fn mt_kahypar_get_partition(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        partition: *mut mt_kahypar_partition_id_t,
    );
}
extern "C" {
    #[doc = " Extracts the weight of each block from a partition. The size of the provided array must be at least the number of blocks."]
    pub fn mt_kahypar_get_block_weights(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        block_weights: *mut mt_kahypar_hypernode_weight_t,
    );
}
extern "C" {
    #[doc = " Computes the imbalance of the partition."]
    pub fn mt_kahypar_imbalance(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        context: *const mt_kahypar_context_t,
    ) -> f64;
}
extern "C" {
    #[doc = " Computes the cut metric."]
    pub fn mt_kahypar_cut(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
    ) -> mt_kahypar_hyperedge_weight_t;
}
extern "C" {
    #[doc = " Computes the connectivity metric."]
    pub fn mt_kahypar_km1(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
    ) -> mt_kahypar_hyperedge_weight_t;
}
extern "C" {
    #[doc = " Computes the sum-of-external-degree metric."]
    pub fn mt_kahypar_soed(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
    ) -> mt_kahypar_hyperedge_weight_t;
}
extern "C" {
    #[doc = " Computes the steiner tree metric."]
    pub fn mt_kahypar_steiner_tree(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
        target_graph: *mut mt_kahypar_target_graph_t,
    ) -> mt_kahypar_hyperedge_weight_t;
}
extern "C" {
    #[doc = " Deletes the partitioned (hyper)graph object."]
    pub fn mt_kahypar_free_partitioned_hypergraph(
        partitioned_hg: mt_kahypar_partitioned_hypergraph_t,
    );
}
