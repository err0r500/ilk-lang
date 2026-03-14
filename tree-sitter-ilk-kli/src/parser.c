#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 262
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 128
#define ALIAS_COUNT 0
#define TOKEN_COUNT 63
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 6
#define MAX_ALIAS_SEQUENCE_LENGTH 8
#define PRODUCTION_ID_COUNT 19

enum ts_symbol_identifiers {
  sym_comment = 1,
  sym_identifier = 2,
  sym_string = 3,
  sym_number = 4,
  anon_sym_true = 5,
  anon_sym_false = 6,
  anon_sym_STAR = 7,
  anon_sym_Uuid = 8,
  anon_sym_String = 9,
  anon_sym_Int = 10,
  anon_sym_Float = 11,
  anon_sym_Bool = 12,
  anon_sym_Date = 13,
  anon_sym_Timestamp = 14,
  anon_sym_Money = 15,
  anon_sym_Concrete = 16,
  anon_sym_LT = 17,
  anon_sym_GT = 18,
  anon_sym_LBRACK = 19,
  anon_sym_RBRACK = 20,
  anon_sym_DOT_DOT = 21,
  anon_sym_AMP = 22,
  anon_sym_DASH = 23,
  anon_sym_LBRACE = 24,
  anon_sym_DOT_DOT_DOT = 25,
  anon_sym_RBRACE = 26,
  anon_sym_COMMA = 27,
  anon_sym__ = 28,
  anon_sym_QMARK = 29,
  anon_sym_PIPE = 30,
  anon_sym_AT = 31,
  anon_sym_main = 32,
  anon_sym_out = 33,
  anon_sym_assoc = 34,
  anon_sym_source = 35,
  anon_sym_constraint = 36,
  anon_sym_doc = 37,
  anon_sym_DOT = 38,
  anon_sym_PIPE_PIPE = 39,
  anon_sym_AMP_AMP = 40,
  anon_sym_BANG = 41,
  anon_sym_EQ_EQ = 42,
  anon_sym_BANG_EQ = 43,
  anon_sym_LT_EQ = 44,
  anon_sym_GT_EQ = 45,
  anon_sym_in = 46,
  anon_sym_LPAREN = 47,
  anon_sym_RPAREN = 48,
  anon_sym_forall = 49,
  anon_sym_exists = 50,
  anon_sym_unique = 51,
  anon_sym_EQ_GT = 52,
  anon_sym_count = 53,
  anon_sym_templateVars = 54,
  anon_sym_keys = 55,
  anon_sym_type = 56,
  anon_sym_EQ = 57,
  sym_main_annotation = 58,
  anon_sym_ATdoc = 59,
  anon_sym_import = 60,
  anon_sym_as = 61,
  anon_sym_compute = 62,
  sym_source_file = 63,
  sym__definition = 64,
  sym_boolean = 65,
  sym_base_type = 66,
  sym__type_expr = 67,
  sym__primary_type = 68,
  sym_named_type = 69,
  sym_literal_type = 70,
  sym_concrete_type = 71,
  sym_list_type = 72,
  sym_cardinality = 73,
  sym_reference_type = 74,
  sym_refinable_ref = 75,
  sym_struct_type = 76,
  sym_anonymous_fields = 77,
  sym_anonymous_field = 78,
  sym_field_list = 79,
  sym_field = 80,
  sym_union_type = 81,
  sym_intersection_type = 82,
  sym_annotation = 83,
  sym_annotation_args = 84,
  sym_source_args = 85,
  sym_source_path = 86,
  sym__constraint_expr = 87,
  sym_constraint_or = 88,
  sym_constraint_and = 89,
  sym_constraint_not = 90,
  sym_constraint_comparison = 91,
  sym__constraint_primary = 92,
  sym_constraint_paren = 93,
  sym_constraint_call = 94,
  sym_constraint_field_access = 95,
  sym_constraint_var = 96,
  sym_constraint_literal = 97,
  sym_type_decl = 98,
  sym_instance = 99,
  sym_doc_annotation = 100,
  sym_assoc_list = 101,
  sym_import_stmt = 102,
  sym__value = 103,
  sym_type_ref = 104,
  sym_literal_value = 105,
  sym_binding_ref = 106,
  sym_struct_value = 107,
  sym_value_field_list = 108,
  sym_value_field = 109,
  sym_field_origin = 110,
  sym_origin_path = 111,
  sym_origin_paths = 112,
  sym_list_value = 113,
  sym_list_elements = 114,
  sym_list_element = 115,
  sym_refinement = 116,
  sym_variant_value = 117,
  aux_sym_source_file_repeat1 = 118,
  aux_sym_anonymous_fields_repeat1 = 119,
  aux_sym_field_list_repeat1 = 120,
  aux_sym_field_repeat1 = 121,
  aux_sym_annotation_args_repeat1 = 122,
  aux_sym_source_args_repeat1 = 123,
  aux_sym_source_path_repeat1 = 124,
  aux_sym_value_field_list_repeat1 = 125,
  aux_sym_origin_paths_repeat1 = 126,
  aux_sym_list_elements_repeat1 = 127,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_comment] = "comment",
  [sym_identifier] = "identifier",
  [sym_string] = "string",
  [sym_number] = "number",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [anon_sym_STAR] = "*",
  [anon_sym_Uuid] = "Uuid",
  [anon_sym_String] = "String",
  [anon_sym_Int] = "Int",
  [anon_sym_Float] = "Float",
  [anon_sym_Bool] = "Bool",
  [anon_sym_Date] = "Date",
  [anon_sym_Timestamp] = "Timestamp",
  [anon_sym_Money] = "Money",
  [anon_sym_Concrete] = "Concrete",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_DOT_DOT] = "..",
  [anon_sym_AMP] = "&",
  [anon_sym_DASH] = "-",
  [anon_sym_LBRACE] = "{",
  [anon_sym_DOT_DOT_DOT] = "...",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COMMA] = ",",
  [anon_sym__] = "_",
  [anon_sym_QMARK] = "\?",
  [anon_sym_PIPE] = "|",
  [anon_sym_AT] = "@",
  [anon_sym_main] = "main",
  [anon_sym_out] = "out",
  [anon_sym_assoc] = "assoc",
  [anon_sym_source] = "source",
  [anon_sym_constraint] = "constraint",
  [anon_sym_doc] = "doc",
  [anon_sym_DOT] = ".",
  [anon_sym_PIPE_PIPE] = "||",
  [anon_sym_AMP_AMP] = "&&",
  [anon_sym_BANG] = "!",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_in] = "in",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_forall] = "forall",
  [anon_sym_exists] = "exists",
  [anon_sym_unique] = "unique",
  [anon_sym_EQ_GT] = "=>",
  [anon_sym_count] = "count",
  [anon_sym_templateVars] = "templateVars",
  [anon_sym_keys] = "keys",
  [anon_sym_type] = "type",
  [anon_sym_EQ] = "=",
  [sym_main_annotation] = "main_annotation",
  [anon_sym_ATdoc] = "@doc",
  [anon_sym_import] = "import",
  [anon_sym_as] = "as",
  [anon_sym_compute] = "compute",
  [sym_source_file] = "source_file",
  [sym__definition] = "_definition",
  [sym_boolean] = "boolean",
  [sym_base_type] = "base_type",
  [sym__type_expr] = "_type_expr",
  [sym__primary_type] = "_primary_type",
  [sym_named_type] = "named_type",
  [sym_literal_type] = "literal_type",
  [sym_concrete_type] = "concrete_type",
  [sym_list_type] = "list_type",
  [sym_cardinality] = "cardinality",
  [sym_reference_type] = "reference_type",
  [sym_refinable_ref] = "refinable_ref",
  [sym_struct_type] = "struct_type",
  [sym_anonymous_fields] = "anonymous_fields",
  [sym_anonymous_field] = "anonymous_field",
  [sym_field_list] = "field_list",
  [sym_field] = "field",
  [sym_union_type] = "union_type",
  [sym_intersection_type] = "intersection_type",
  [sym_annotation] = "annotation",
  [sym_annotation_args] = "annotation_args",
  [sym_source_args] = "source_args",
  [sym_source_path] = "source_path",
  [sym__constraint_expr] = "_constraint_expr",
  [sym_constraint_or] = "constraint_or",
  [sym_constraint_and] = "constraint_and",
  [sym_constraint_not] = "constraint_not",
  [sym_constraint_comparison] = "constraint_comparison",
  [sym__constraint_primary] = "_constraint_primary",
  [sym_constraint_paren] = "constraint_paren",
  [sym_constraint_call] = "constraint_call",
  [sym_constraint_field_access] = "constraint_field_access",
  [sym_constraint_var] = "constraint_var",
  [sym_constraint_literal] = "constraint_literal",
  [sym_type_decl] = "type_decl",
  [sym_instance] = "instance",
  [sym_doc_annotation] = "doc_annotation",
  [sym_assoc_list] = "assoc_list",
  [sym_import_stmt] = "import_stmt",
  [sym__value] = "_value",
  [sym_type_ref] = "type_ref",
  [sym_literal_value] = "literal_value",
  [sym_binding_ref] = "binding_ref",
  [sym_struct_value] = "struct_value",
  [sym_value_field_list] = "value_field_list",
  [sym_value_field] = "value_field",
  [sym_field_origin] = "field_origin",
  [sym_origin_path] = "origin_path",
  [sym_origin_paths] = "origin_paths",
  [sym_list_value] = "list_value",
  [sym_list_elements] = "list_elements",
  [sym_list_element] = "list_element",
  [sym_refinement] = "refinement",
  [sym_variant_value] = "variant_value",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_anonymous_fields_repeat1] = "anonymous_fields_repeat1",
  [aux_sym_field_list_repeat1] = "field_list_repeat1",
  [aux_sym_field_repeat1] = "field_repeat1",
  [aux_sym_annotation_args_repeat1] = "annotation_args_repeat1",
  [aux_sym_source_args_repeat1] = "source_args_repeat1",
  [aux_sym_source_path_repeat1] = "source_path_repeat1",
  [aux_sym_value_field_list_repeat1] = "value_field_list_repeat1",
  [aux_sym_origin_paths_repeat1] = "origin_paths_repeat1",
  [aux_sym_list_elements_repeat1] = "list_elements_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_comment] = sym_comment,
  [sym_identifier] = sym_identifier,
  [sym_string] = sym_string,
  [sym_number] = sym_number,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_Uuid] = anon_sym_Uuid,
  [anon_sym_String] = anon_sym_String,
  [anon_sym_Int] = anon_sym_Int,
  [anon_sym_Float] = anon_sym_Float,
  [anon_sym_Bool] = anon_sym_Bool,
  [anon_sym_Date] = anon_sym_Date,
  [anon_sym_Timestamp] = anon_sym_Timestamp,
  [anon_sym_Money] = anon_sym_Money,
  [anon_sym_Concrete] = anon_sym_Concrete,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_DOT_DOT] = anon_sym_DOT_DOT,
  [anon_sym_AMP] = anon_sym_AMP,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_DOT_DOT_DOT] = anon_sym_DOT_DOT_DOT,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym__] = anon_sym__,
  [anon_sym_QMARK] = anon_sym_QMARK,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_main] = anon_sym_main,
  [anon_sym_out] = anon_sym_out,
  [anon_sym_assoc] = anon_sym_assoc,
  [anon_sym_source] = anon_sym_source,
  [anon_sym_constraint] = anon_sym_constraint,
  [anon_sym_doc] = anon_sym_doc,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_PIPE_PIPE] = anon_sym_PIPE_PIPE,
  [anon_sym_AMP_AMP] = anon_sym_AMP_AMP,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_in] = anon_sym_in,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_forall] = anon_sym_forall,
  [anon_sym_exists] = anon_sym_exists,
  [anon_sym_unique] = anon_sym_unique,
  [anon_sym_EQ_GT] = anon_sym_EQ_GT,
  [anon_sym_count] = anon_sym_count,
  [anon_sym_templateVars] = anon_sym_templateVars,
  [anon_sym_keys] = anon_sym_keys,
  [anon_sym_type] = anon_sym_type,
  [anon_sym_EQ] = anon_sym_EQ,
  [sym_main_annotation] = sym_main_annotation,
  [anon_sym_ATdoc] = anon_sym_ATdoc,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_as] = anon_sym_as,
  [anon_sym_compute] = anon_sym_compute,
  [sym_source_file] = sym_source_file,
  [sym__definition] = sym__definition,
  [sym_boolean] = sym_boolean,
  [sym_base_type] = sym_base_type,
  [sym__type_expr] = sym__type_expr,
  [sym__primary_type] = sym__primary_type,
  [sym_named_type] = sym_named_type,
  [sym_literal_type] = sym_literal_type,
  [sym_concrete_type] = sym_concrete_type,
  [sym_list_type] = sym_list_type,
  [sym_cardinality] = sym_cardinality,
  [sym_reference_type] = sym_reference_type,
  [sym_refinable_ref] = sym_refinable_ref,
  [sym_struct_type] = sym_struct_type,
  [sym_anonymous_fields] = sym_anonymous_fields,
  [sym_anonymous_field] = sym_anonymous_field,
  [sym_field_list] = sym_field_list,
  [sym_field] = sym_field,
  [sym_union_type] = sym_union_type,
  [sym_intersection_type] = sym_intersection_type,
  [sym_annotation] = sym_annotation,
  [sym_annotation_args] = sym_annotation_args,
  [sym_source_args] = sym_source_args,
  [sym_source_path] = sym_source_path,
  [sym__constraint_expr] = sym__constraint_expr,
  [sym_constraint_or] = sym_constraint_or,
  [sym_constraint_and] = sym_constraint_and,
  [sym_constraint_not] = sym_constraint_not,
  [sym_constraint_comparison] = sym_constraint_comparison,
  [sym__constraint_primary] = sym__constraint_primary,
  [sym_constraint_paren] = sym_constraint_paren,
  [sym_constraint_call] = sym_constraint_call,
  [sym_constraint_field_access] = sym_constraint_field_access,
  [sym_constraint_var] = sym_constraint_var,
  [sym_constraint_literal] = sym_constraint_literal,
  [sym_type_decl] = sym_type_decl,
  [sym_instance] = sym_instance,
  [sym_doc_annotation] = sym_doc_annotation,
  [sym_assoc_list] = sym_assoc_list,
  [sym_import_stmt] = sym_import_stmt,
  [sym__value] = sym__value,
  [sym_type_ref] = sym_type_ref,
  [sym_literal_value] = sym_literal_value,
  [sym_binding_ref] = sym_binding_ref,
  [sym_struct_value] = sym_struct_value,
  [sym_value_field_list] = sym_value_field_list,
  [sym_value_field] = sym_value_field,
  [sym_field_origin] = sym_field_origin,
  [sym_origin_path] = sym_origin_path,
  [sym_origin_paths] = sym_origin_paths,
  [sym_list_value] = sym_list_value,
  [sym_list_elements] = sym_list_elements,
  [sym_list_element] = sym_list_element,
  [sym_refinement] = sym_refinement,
  [sym_variant_value] = sym_variant_value,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_anonymous_fields_repeat1] = aux_sym_anonymous_fields_repeat1,
  [aux_sym_field_list_repeat1] = aux_sym_field_list_repeat1,
  [aux_sym_field_repeat1] = aux_sym_field_repeat1,
  [aux_sym_annotation_args_repeat1] = aux_sym_annotation_args_repeat1,
  [aux_sym_source_args_repeat1] = aux_sym_source_args_repeat1,
  [aux_sym_source_path_repeat1] = aux_sym_source_path_repeat1,
  [aux_sym_value_field_list_repeat1] = aux_sym_value_field_list_repeat1,
  [aux_sym_origin_paths_repeat1] = aux_sym_origin_paths_repeat1,
  [aux_sym_list_elements_repeat1] = aux_sym_list_elements_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Uuid] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_String] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Int] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Float] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Bool] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Date] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Timestamp] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Money] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Concrete] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT_DOT_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym__] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_QMARK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_main] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_out] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_assoc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_source] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_constraint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_doc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_in] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_forall] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_exists] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_unique] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_count] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_templateVars] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_keys] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_type] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [sym_main_annotation] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_ATdoc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_as] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_compute] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__definition] = {
    .visible = false,
    .named = true,
  },
  [sym_boolean] = {
    .visible = true,
    .named = true,
  },
  [sym_base_type] = {
    .visible = true,
    .named = true,
  },
  [sym__type_expr] = {
    .visible = false,
    .named = true,
  },
  [sym__primary_type] = {
    .visible = false,
    .named = true,
  },
  [sym_named_type] = {
    .visible = true,
    .named = true,
  },
  [sym_literal_type] = {
    .visible = true,
    .named = true,
  },
  [sym_concrete_type] = {
    .visible = true,
    .named = true,
  },
  [sym_list_type] = {
    .visible = true,
    .named = true,
  },
  [sym_cardinality] = {
    .visible = true,
    .named = true,
  },
  [sym_reference_type] = {
    .visible = true,
    .named = true,
  },
  [sym_refinable_ref] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_type] = {
    .visible = true,
    .named = true,
  },
  [sym_anonymous_fields] = {
    .visible = true,
    .named = true,
  },
  [sym_anonymous_field] = {
    .visible = true,
    .named = true,
  },
  [sym_field_list] = {
    .visible = true,
    .named = true,
  },
  [sym_field] = {
    .visible = true,
    .named = true,
  },
  [sym_union_type] = {
    .visible = true,
    .named = true,
  },
  [sym_intersection_type] = {
    .visible = true,
    .named = true,
  },
  [sym_annotation] = {
    .visible = true,
    .named = true,
  },
  [sym_annotation_args] = {
    .visible = true,
    .named = true,
  },
  [sym_source_args] = {
    .visible = true,
    .named = true,
  },
  [sym_source_path] = {
    .visible = true,
    .named = true,
  },
  [sym__constraint_expr] = {
    .visible = false,
    .named = true,
  },
  [sym_constraint_or] = {
    .visible = true,
    .named = true,
  },
  [sym_constraint_and] = {
    .visible = true,
    .named = true,
  },
  [sym_constraint_not] = {
    .visible = true,
    .named = true,
  },
  [sym_constraint_comparison] = {
    .visible = true,
    .named = true,
  },
  [sym__constraint_primary] = {
    .visible = false,
    .named = true,
  },
  [sym_constraint_paren] = {
    .visible = true,
    .named = true,
  },
  [sym_constraint_call] = {
    .visible = true,
    .named = true,
  },
  [sym_constraint_field_access] = {
    .visible = true,
    .named = true,
  },
  [sym_constraint_var] = {
    .visible = true,
    .named = true,
  },
  [sym_constraint_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_type_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_instance] = {
    .visible = true,
    .named = true,
  },
  [sym_doc_annotation] = {
    .visible = true,
    .named = true,
  },
  [sym_assoc_list] = {
    .visible = true,
    .named = true,
  },
  [sym_import_stmt] = {
    .visible = true,
    .named = true,
  },
  [sym__value] = {
    .visible = false,
    .named = true,
  },
  [sym_type_ref] = {
    .visible = true,
    .named = true,
  },
  [sym_literal_value] = {
    .visible = true,
    .named = true,
  },
  [sym_binding_ref] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_value] = {
    .visible = true,
    .named = true,
  },
  [sym_value_field_list] = {
    .visible = true,
    .named = true,
  },
  [sym_value_field] = {
    .visible = true,
    .named = true,
  },
  [sym_field_origin] = {
    .visible = true,
    .named = true,
  },
  [sym_origin_path] = {
    .visible = true,
    .named = true,
  },
  [sym_origin_paths] = {
    .visible = true,
    .named = true,
  },
  [sym_list_value] = {
    .visible = true,
    .named = true,
  },
  [sym_list_elements] = {
    .visible = true,
    .named = true,
  },
  [sym_list_element] = {
    .visible = true,
    .named = true,
  },
  [sym_refinement] = {
    .visible = true,
    .named = true,
  },
  [sym_variant_value] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_anonymous_fields_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_field_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_field_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_annotation_args_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_source_args_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_source_path_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_value_field_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_origin_paths_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_list_elements_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_alias = 1,
  field_body = 2,
  field_name = 3,
  field_path = 4,
  field_type = 5,
  field_value = 6,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_alias] = "alias",
  [field_body] = "body",
  [field_name] = "name",
  [field_path] = "path",
  [field_type] = "type",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 3},
  [3] = {.index = 4, .length = 2},
  [4] = {.index = 6, .length = 2},
  [5] = {.index = 8, .length = 3},
  [6] = {.index = 11, .length = 3},
  [7] = {.index = 14, .length = 2},
  [8] = {.index = 16, .length = 2},
  [9] = {.index = 18, .length = 2},
  [10] = {.index = 20, .length = 3},
  [11] = {.index = 23, .length = 3},
  [12] = {.index = 26, .length = 2},
  [13] = {.index = 28, .length = 2},
  [14] = {.index = 30, .length = 2},
  [15] = {.index = 32, .length = 2},
  [16] = {.index = 34, .length = 3},
  [17] = {.index = 37, .length = 2},
  [18] = {.index = 39, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_path, 1},
  [1] =
    {field_name, 0},
    {field_type, 2},
    {field_value, 3},
  [4] =
    {field_body, 3},
    {field_name, 1},
  [6] =
    {field_alias, 3},
    {field_path, 1},
  [8] =
    {field_name, 0},
    {field_type, 2},
    {field_value, 4},
  [11] =
    {field_name, 1},
    {field_type, 3},
    {field_value, 4},
  [14] =
    {field_body, 4},
    {field_name, 2},
  [16] =
    {field_name, 0},
    {field_value, 1},
  [18] =
    {field_name, 0},
    {field_type, 1},
  [20] =
    {field_name, 1},
    {field_type, 3},
    {field_value, 5},
  [23] =
    {field_name, 2},
    {field_type, 4},
    {field_value, 5},
  [26] =
    {field_name, 0},
    {field_value, 2},
  [28] =
    {field_name, 1},
    {field_value, 2},
  [30] =
    {field_name, 0},
    {field_type, 2},
  [32] =
    {field_name, 1},
    {field_type, 2},
  [34] =
    {field_name, 2},
    {field_type, 4},
    {field_value, 6},
  [37] =
    {field_name, 1},
    {field_value, 3},
  [39] =
    {field_name, 1},
    {field_type, 3},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 14,
  [17] = 15,
  [18] = 18,
  [19] = 11,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 11,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 28,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 27,
  [42] = 42,
  [43] = 43,
  [44] = 30,
  [45] = 35,
  [46] = 36,
  [47] = 37,
  [48] = 31,
  [49] = 39,
  [50] = 34,
  [51] = 43,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 63,
  [84] = 65,
  [85] = 52,
  [86] = 86,
  [87] = 69,
  [88] = 61,
  [89] = 89,
  [90] = 68,
  [91] = 62,
  [92] = 58,
  [93] = 59,
  [94] = 60,
  [95] = 64,
  [96] = 66,
  [97] = 67,
  [98] = 52,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 102,
  [107] = 104,
  [108] = 108,
  [109] = 109,
  [110] = 109,
  [111] = 105,
  [112] = 108,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 123,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 143,
  [146] = 146,
  [147] = 147,
  [148] = 146,
  [149] = 149,
  [150] = 150,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 156,
  [157] = 157,
  [158] = 158,
  [159] = 159,
  [160] = 160,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 164,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 169,
  [170] = 170,
  [171] = 171,
  [172] = 172,
  [173] = 173,
  [174] = 174,
  [175] = 175,
  [176] = 171,
  [177] = 177,
  [178] = 178,
  [179] = 179,
  [180] = 180,
  [181] = 181,
  [182] = 182,
  [183] = 183,
  [184] = 184,
  [185] = 185,
  [186] = 186,
  [187] = 187,
  [188] = 188,
  [189] = 189,
  [190] = 190,
  [191] = 191,
  [192] = 192,
  [193] = 192,
  [194] = 194,
  [195] = 195,
  [196] = 183,
  [197] = 197,
  [198] = 189,
  [199] = 199,
  [200] = 200,
  [201] = 201,
  [202] = 202,
  [203] = 203,
  [204] = 191,
  [205] = 205,
  [206] = 201,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 210,
  [211] = 211,
  [212] = 212,
  [213] = 213,
  [214] = 214,
  [215] = 215,
  [216] = 216,
  [217] = 217,
  [218] = 218,
  [219] = 219,
  [220] = 220,
  [221] = 221,
  [222] = 222,
  [223] = 223,
  [224] = 224,
  [225] = 225,
  [226] = 226,
  [227] = 227,
  [228] = 228,
  [229] = 229,
  [230] = 230,
  [231] = 231,
  [232] = 232,
  [233] = 233,
  [234] = 234,
  [235] = 235,
  [236] = 236,
  [237] = 237,
  [238] = 233,
  [239] = 210,
  [240] = 240,
  [241] = 241,
  [242] = 242,
  [243] = 225,
  [244] = 244,
  [245] = 245,
  [246] = 246,
  [247] = 247,
  [248] = 237,
  [249] = 249,
  [250] = 250,
  [251] = 251,
  [252] = 232,
  [253] = 253,
  [254] = 211,
  [255] = 215,
  [256] = 220,
  [257] = 216,
  [258] = 258,
  [259] = 259,
  [260] = 212,
  [261] = 261,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(149);
      ADVANCE_MAP(
        '!', 310,
        '"', 7,
        '&', 285,
        '(', 317,
        ')', 318,
        '*', 258,
        ',', 290,
        '-', 286,
        '.', 306,
        '/', 17,
        '<', 278,
        '=', 336,
        '>', 280,
        '?', 293,
        '@', 297,
        'B', 93,
        'C', 88,
        'D', 23,
        'F', 66,
        'I', 78,
        'M', 90,
        'S', 119,
        'T', 54,
        'U', 133,
        '[', 281,
        ']', 282,
        '_', 291,
        'a', 109,
        'c', 84,
        'd', 85,
        'e', 139,
        'f', 22,
        'i', 68,
        'k', 39,
        'm', 25,
        'o', 135,
        's', 89,
        't', 49,
        'u', 80,
        '{', 287,
        '|', 295,
        '}', 289,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      END_STATE();
    case 1:
      ADVANCE_MAP(
        '!', 309,
        '"', 7,
        '(', 317,
        '-', 144,
        '/', 17,
        'c', 205,
        'e', 247,
        'f', 152,
        'k', 173,
        't', 175,
        'u', 198,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 2:
      ADVANCE_MAP(
        '!', 18,
        '&', 8,
        ')', 318,
        '*', 258,
        ',', 290,
        '.', 305,
        '/', 17,
        '<', 278,
        '=', 335,
        '>', 280,
        '@', 296,
        ']', 282,
        'a', 116,
        'c', 94,
        'd', 85,
        'i', 73,
        'm', 25,
        'o', 135,
        's', 89,
        't', 141,
        '|', 143,
        '}', 289,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2);
      END_STATE();
    case 3:
      ADVANCE_MAP(
        '!', 18,
        '&', 8,
        ')', 318,
        ',', 290,
        '.', 305,
        '/', 17,
        '<', 278,
        '=', 19,
        '>', 280,
        '@', 296,
        'i', 73,
        't', 141,
        '|', 143,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3);
      END_STATE();
    case 4:
      ADVANCE_MAP(
        '!', 18,
        '&', 8,
        '.', 305,
        '/', 17,
        '<', 278,
        '=', 19,
        '>', 280,
        '@', 296,
        'i', 195,
        '|', 143,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(4);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 5:
      ADVANCE_MAP(
        '"', 7,
        '&', 285,
        '*', 258,
        ',', 290,
        '-', 286,
        '.', 10,
        '/', 17,
        '=', 20,
        '?', 293,
        'B', 206,
        'C', 201,
        'D', 154,
        'F', 185,
        'I', 197,
        'M', 204,
        'S', 236,
        'T', 179,
        'U', 242,
        '[', 281,
        ']', 282,
        'f', 153,
        't', 217,
        '{', 287,
        '}', 289,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 6:
      ADVANCE_MAP(
        '"', 7,
        '&', 285,
        '*', 258,
        ',', 290,
        '-', 144,
        '/', 17,
        '<', 277,
        '=', 334,
        '?', 293,
        'B', 206,
        'D', 154,
        'F', 185,
        'I', 197,
        'M', 204,
        'S', 236,
        'T', 179,
        'U', 242,
        '[', 281,
        ']', 282,
        'f', 153,
        't', 217,
        '{', 287,
        '}', 289,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(6);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 7:
      if (lookahead == '"') ADVANCE(252);
      if (lookahead == '\\') ADVANCE(145);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 8:
      if (lookahead == '&') ADVANCE(308);
      END_STATE();
    case 9:
      if (lookahead == '.') ADVANCE(288);
      END_STATE();
    case 10:
      if (lookahead == '.') ADVANCE(283);
      END_STATE();
    case 11:
      if (lookahead == '.') ADVANCE(12);
      if (lookahead == '/') ADVANCE(17);
      if (lookahead == '@') ADVANCE(296);
      if (lookahead == '_') ADVANCE(292);
      if (lookahead == '}') ADVANCE(289);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(11);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 12:
      if (lookahead == '.') ADVANCE(9);
      END_STATE();
    case 13:
      if (lookahead == '/') ADVANCE(17);
      if (lookahead == '@') ADVANCE(296);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(13);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 14:
      if (lookahead == '/') ADVANCE(17);
      if (lookahead == '@') ADVANCE(38);
      if (lookahead == '}') ADVANCE(289);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(14);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 15:
      if (lookahead == '/') ADVANCE(17);
      if (lookahead == 'a') ADVANCE(228);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(15);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 16:
      if (lookahead == '/') ADVANCE(17);
      if (lookahead == 'c') ADVANCE(208);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(16);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 17:
      if (lookahead == '/') ADVANCE(150);
      END_STATE();
    case 18:
      if (lookahead == '=') ADVANCE(312);
      END_STATE();
    case 19:
      if (lookahead == '=') ADVANCE(311);
      END_STATE();
    case 20:
      if (lookahead == '>') ADVANCE(325);
      END_STATE();
    case 21:
      if (lookahead == 'V') ADVANCE(28);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(64);
      if (lookahead == 'o') ADVANCE(102);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(126);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(65);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(58);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(122);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(61);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(104);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(72);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(59);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(132);
      END_STATE();
    case 32:
      if (lookahead == 'c') ADVANCE(304);
      END_STATE();
    case 33:
      if (lookahead == 'c') ADVANCE(338);
      END_STATE();
    case 34:
      if (lookahead == 'c') ADVANCE(300);
      END_STATE();
    case 35:
      if (lookahead == 'c') ADVANCE(105);
      END_STATE();
    case 36:
      if (lookahead == 'c') ADVANCE(44);
      END_STATE();
    case 37:
      if (lookahead == 'd') ADVANCE(259);
      END_STATE();
    case 38:
      if (lookahead == 'd') ADVANCE(86);
      END_STATE();
    case 39:
      if (lookahead == 'e') ADVANCE(142);
      END_STATE();
    case 40:
      if (lookahead == 'e') ADVANCE(269);
      END_STATE();
    case 41:
      if (lookahead == 'e') ADVANCE(254);
      END_STATE();
    case 42:
      if (lookahead == 'e') ADVANCE(332);
      END_STATE();
    case 43:
      if (lookahead == 'e') ADVANCE(256);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(302);
      END_STATE();
    case 45:
      if (lookahead == 'e') ADVANCE(323);
      END_STATE();
    case 46:
      if (lookahead == 'e') ADVANCE(343);
      END_STATE();
    case 47:
      if (lookahead == 'e') ADVANCE(275);
      END_STATE();
    case 48:
      if (lookahead == 'e') ADVANCE(21);
      END_STATE();
    case 49:
      if (lookahead == 'e') ADVANCE(71);
      if (lookahead == 'r') ADVANCE(136);
      if (lookahead == 'y') ADVANCE(99);
      END_STATE();
    case 50:
      if (lookahead == 'e') ADVANCE(140);
      END_STATE();
    case 51:
      if (lookahead == 'e') ADVANCE(115);
      END_STATE();
    case 52:
      if (lookahead == 'e') ADVANCE(131);
      END_STATE();
    case 53:
      if (lookahead == 'g') ADVANCE(261);
      END_STATE();
    case 54:
      if (lookahead == 'i') ADVANCE(69);
      END_STATE();
    case 55:
      if (lookahead == 'i') ADVANCE(37);
      END_STATE();
    case 56:
      if (lookahead == 'i') ADVANCE(101);
      END_STATE();
    case 57:
      if (lookahead == 'i') ADVANCE(114);
      END_STATE();
    case 58:
      if (lookahead == 'i') ADVANCE(74);
      END_STATE();
    case 59:
      if (lookahead == 'i') ADVANCE(75);
      END_STATE();
    case 60:
      if (lookahead == 'i') ADVANCE(76);
      END_STATE();
    case 61:
      if (lookahead == 'i') ADVANCE(82);
      END_STATE();
    case 62:
      if (lookahead == 'l') ADVANCE(267);
      END_STATE();
    case 63:
      if (lookahead == 'l') ADVANCE(319);
      END_STATE();
    case 64:
      if (lookahead == 'l') ADVANCE(113);
      END_STATE();
    case 65:
      if (lookahead == 'l') ADVANCE(63);
      END_STATE();
    case 66:
      if (lookahead == 'l') ADVANCE(95);
      END_STATE();
    case 67:
      if (lookahead == 'l') ADVANCE(31);
      END_STATE();
    case 68:
      if (lookahead == 'm') ADVANCE(100);
      if (lookahead == 'n') ADVANCE(315);
      END_STATE();
    case 69:
      if (lookahead == 'm') ADVANCE(51);
      END_STATE();
    case 70:
      if (lookahead == 'm') ADVANCE(98);
      if (lookahead == 'n') ADVANCE(118);
      if (lookahead == 'u') ADVANCE(81);
      END_STATE();
    case 71:
      if (lookahead == 'm') ADVANCE(97);
      END_STATE();
    case 72:
      if (lookahead == 'm') ADVANCE(96);
      END_STATE();
    case 73:
      if (lookahead == 'n') ADVANCE(315);
      END_STATE();
    case 74:
      if (lookahead == 'n') ADVANCE(298);
      END_STATE();
    case 75:
      if (lookahead == 'n') ADVANCE(337);
      END_STATE();
    case 76:
      if (lookahead == 'n') ADVANCE(53);
      END_STATE();
    case 77:
      if (lookahead == 'n') ADVANCE(118);
      END_STATE();
    case 78:
      if (lookahead == 'n') ADVANCE(120);
      END_STATE();
    case 79:
      if (lookahead == 'n') ADVANCE(35);
      END_STATE();
    case 80:
      if (lookahead == 'n') ADVANCE(56);
      END_STATE();
    case 81:
      if (lookahead == 'n') ADVANCE(123);
      END_STATE();
    case 82:
      if (lookahead == 'n') ADVANCE(125);
      END_STATE();
    case 83:
      if (lookahead == 'n') ADVANCE(50);
      END_STATE();
    case 84:
      if (lookahead == 'o') ADVANCE(70);
      END_STATE();
    case 85:
      if (lookahead == 'o') ADVANCE(32);
      END_STATE();
    case 86:
      if (lookahead == 'o') ADVANCE(33);
      END_STATE();
    case 87:
      if (lookahead == 'o') ADVANCE(62);
      END_STATE();
    case 88:
      if (lookahead == 'o') ADVANCE(79);
      END_STATE();
    case 89:
      if (lookahead == 'o') ADVANCE(134);
      END_STATE();
    case 90:
      if (lookahead == 'o') ADVANCE(83);
      END_STATE();
    case 91:
      if (lookahead == 'o') ADVANCE(106);
      END_STATE();
    case 92:
      if (lookahead == 'o') ADVANCE(34);
      END_STATE();
    case 93:
      if (lookahead == 'o') ADVANCE(87);
      END_STATE();
    case 94:
      if (lookahead == 'o') ADVANCE(77);
      END_STATE();
    case 95:
      if (lookahead == 'o') ADVANCE(26);
      END_STATE();
    case 96:
      if (lookahead == 'p') ADVANCE(271);
      END_STATE();
    case 97:
      if (lookahead == 'p') ADVANCE(67);
      END_STATE();
    case 98:
      if (lookahead == 'p') ADVANCE(138);
      END_STATE();
    case 99:
      if (lookahead == 'p') ADVANCE(42);
      END_STATE();
    case 100:
      if (lookahead == 'p') ADVANCE(91);
      END_STATE();
    case 101:
      if (lookahead == 'q') ADVANCE(137);
      END_STATE();
    case 102:
      if (lookahead == 'r') ADVANCE(24);
      END_STATE();
    case 103:
      if (lookahead == 'r') ADVANCE(36);
      END_STATE();
    case 104:
      if (lookahead == 'r') ADVANCE(112);
      END_STATE();
    case 105:
      if (lookahead == 'r') ADVANCE(52);
      END_STATE();
    case 106:
      if (lookahead == 'r') ADVANCE(124);
      END_STATE();
    case 107:
      if (lookahead == 'r') ADVANCE(27);
      END_STATE();
    case 108:
      if (lookahead == 'r') ADVANCE(60);
      END_STATE();
    case 109:
      if (lookahead == 's') ADVANCE(341);
      END_STATE();
    case 110:
      if (lookahead == 's') ADVANCE(330);
      END_STATE();
    case 111:
      if (lookahead == 's') ADVANCE(321);
      END_STATE();
    case 112:
      if (lookahead == 's') ADVANCE(328);
      END_STATE();
    case 113:
      if (lookahead == 's') ADVANCE(43);
      END_STATE();
    case 114:
      if (lookahead == 's') ADVANCE(128);
      END_STATE();
    case 115:
      if (lookahead == 's') ADVANCE(127);
      END_STATE();
    case 116:
      if (lookahead == 's') ADVANCE(117);
      END_STATE();
    case 117:
      if (lookahead == 's') ADVANCE(92);
      END_STATE();
    case 118:
      if (lookahead == 's') ADVANCE(129);
      END_STATE();
    case 119:
      if (lookahead == 't') ADVANCE(108);
      END_STATE();
    case 120:
      if (lookahead == 't') ADVANCE(263);
      END_STATE();
    case 121:
      if (lookahead == 't') ADVANCE(299);
      END_STATE();
    case 122:
      if (lookahead == 't') ADVANCE(265);
      END_STATE();
    case 123:
      if (lookahead == 't') ADVANCE(326);
      END_STATE();
    case 124:
      if (lookahead == 't') ADVANCE(339);
      END_STATE();
    case 125:
      if (lookahead == 't') ADVANCE(303);
      END_STATE();
    case 126:
      if (lookahead == 't') ADVANCE(40);
      END_STATE();
    case 127:
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 128:
      if (lookahead == 't') ADVANCE(111);
      END_STATE();
    case 129:
      if (lookahead == 't') ADVANCE(107);
      END_STATE();
    case 130:
      if (lookahead == 't') ADVANCE(46);
      END_STATE();
    case 131:
      if (lookahead == 't') ADVANCE(47);
      END_STATE();
    case 132:
      if (lookahead == 't') ADVANCE(48);
      END_STATE();
    case 133:
      if (lookahead == 'u') ADVANCE(55);
      END_STATE();
    case 134:
      if (lookahead == 'u') ADVANCE(103);
      END_STATE();
    case 135:
      if (lookahead == 'u') ADVANCE(121);
      END_STATE();
    case 136:
      if (lookahead == 'u') ADVANCE(41);
      END_STATE();
    case 137:
      if (lookahead == 'u') ADVANCE(45);
      END_STATE();
    case 138:
      if (lookahead == 'u') ADVANCE(130);
      END_STATE();
    case 139:
      if (lookahead == 'x') ADVANCE(57);
      END_STATE();
    case 140:
      if (lookahead == 'y') ADVANCE(273);
      END_STATE();
    case 141:
      if (lookahead == 'y') ADVANCE(99);
      END_STATE();
    case 142:
      if (lookahead == 'y') ADVANCE(110);
      END_STATE();
    case 143:
      if (lookahead == '|') ADVANCE(307);
      END_STATE();
    case 144:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      END_STATE();
    case 145:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(7);
      END_STATE();
    case 146:
      if (eof) ADVANCE(149);
      ADVANCE_MAP(
        '"', 7,
        '*', 258,
        '-', 144,
        '/', 17,
        '@', 297,
        'B', 206,
        'D', 154,
        'F', 185,
        'I', 197,
        'M', 204,
        'S', 236,
        'T', 179,
        'U', 242,
        '[', 281,
        'f', 153,
        'i', 188,
        't', 216,
        '{', 287,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(146);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 147:
      if (eof) ADVANCE(149);
      ADVANCE_MAP(
        '&', 285,
        ')', 318,
        '*', 258,
        ',', 290,
        '.', 305,
        '/', 17,
        '=', 334,
        '>', 279,
        '@', 297,
        ']', 282,
        'i', 188,
        't', 249,
        '|', 294,
        '}', 289,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(147);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 148:
      if (eof) ADVANCE(149);
      if (lookahead == '/') ADVANCE(17);
      if (lookahead == '@') ADVANCE(297);
      if (lookahead == 'a') ADVANCE(225);
      if (lookahead == 'i') ADVANCE(188);
      if (lookahead == 't') ADVANCE(249);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(148);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(150);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'V') ADVANCE(157);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(182);
      if (lookahead == 'o') ADVANCE(221);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(182);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(235);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(186);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(233);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(219);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(189);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(240);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(301);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 161:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(220);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'd') ADVANCE(260);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(333);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(270);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(255);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(257);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(276);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(324);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(151);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(344);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(248);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(227);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(250);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(239);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(190);
      if (lookahead == 'r') ADVANCE(243);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(262);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(162);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(214);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(191);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(194);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(230);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(226);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(268);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(320);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(207);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(184);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(159);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(209);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 189:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(211);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(213);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(172);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(212);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(161);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(176);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(316);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(171);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(232);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 198:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(178);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(234);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 200:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(215);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(193);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 202:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(160);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 203:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(183);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(196);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(245);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(203);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(156);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(192);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 209:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(200);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(163);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(272);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(246);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(187);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'q') ADVANCE(244);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(231);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(243);
      if (lookahead == 'y') ADVANCE(210);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(243);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(180);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(224);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(174);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(155);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 222:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(331);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(322);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 224:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(329);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(342);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(166);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 227:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(237);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 228:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(229);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 229:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(202);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(238);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(340);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(264);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(266);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 234:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(327);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 235:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(164);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(218);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(158);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(223);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(167);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(169);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(170);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(177);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 243:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(165);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(168);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(199);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 246:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(241);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 247:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'x') ADVANCE(181);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'y') ADVANCE(274);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'y') ADVANCE(210);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 250:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'y') ADVANCE(222);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 252:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 253:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 255:
      ACCEPT_TOKEN(anon_sym_true);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(anon_sym_false);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 258:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 259:
      ACCEPT_TOKEN(anon_sym_Uuid);
      END_STATE();
    case 260:
      ACCEPT_TOKEN(anon_sym_Uuid);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(anon_sym_String);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(anon_sym_String);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 263:
      ACCEPT_TOKEN(anon_sym_Int);
      END_STATE();
    case 264:
      ACCEPT_TOKEN(anon_sym_Int);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 265:
      ACCEPT_TOKEN(anon_sym_Float);
      END_STATE();
    case 266:
      ACCEPT_TOKEN(anon_sym_Float);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 267:
      ACCEPT_TOKEN(anon_sym_Bool);
      END_STATE();
    case 268:
      ACCEPT_TOKEN(anon_sym_Bool);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(anon_sym_Date);
      END_STATE();
    case 270:
      ACCEPT_TOKEN(anon_sym_Date);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(anon_sym_Timestamp);
      END_STATE();
    case 272:
      ACCEPT_TOKEN(anon_sym_Timestamp);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 273:
      ACCEPT_TOKEN(anon_sym_Money);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(anon_sym_Money);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(anon_sym_Concrete);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(anon_sym_Concrete);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(313);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(314);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      if (lookahead == '.') ADVANCE(288);
      END_STATE();
    case 285:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 286:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(253);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_DOT_DOT_DOT);
      END_STATE();
    case 289:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 290:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(anon_sym__);
      END_STATE();
    case 292:
      ACCEPT_TOKEN(anon_sym__);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(anon_sym_QMARK);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(anon_sym_PIPE);
      if (lookahead == '|') ADVANCE(307);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead == 'd') ADVANCE(86);
      if (lookahead == 'm') ADVANCE(30);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(anon_sym_main);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(anon_sym_out);
      END_STATE();
    case 300:
      ACCEPT_TOKEN(anon_sym_assoc);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(anon_sym_assoc);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(anon_sym_source);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(anon_sym_constraint);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(anon_sym_doc);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 306:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead == '.') ADVANCE(284);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(anon_sym_PIPE_PIPE);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_AMP_AMP);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(312);
      END_STATE();
    case 311:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 312:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 314:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 315:
      ACCEPT_TOKEN(anon_sym_in);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(anon_sym_in);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 318:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 319:
      ACCEPT_TOKEN(anon_sym_forall);
      END_STATE();
    case 320:
      ACCEPT_TOKEN(anon_sym_forall);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 321:
      ACCEPT_TOKEN(anon_sym_exists);
      END_STATE();
    case 322:
      ACCEPT_TOKEN(anon_sym_exists);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 323:
      ACCEPT_TOKEN(anon_sym_unique);
      END_STATE();
    case 324:
      ACCEPT_TOKEN(anon_sym_unique);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(anon_sym_EQ_GT);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(anon_sym_count);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(anon_sym_templateVars);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(anon_sym_templateVars);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(anon_sym_keys);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(anon_sym_keys);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(anon_sym_type);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(anon_sym_type);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(311);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(311);
      if (lookahead == '>') ADVANCE(325);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(sym_main_annotation);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(anon_sym_ATdoc);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(anon_sym_import);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(anon_sym_as);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(anon_sym_as);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    case 343:
      ACCEPT_TOKEN(anon_sym_compute);
      END_STATE();
    case 344:
      ACCEPT_TOKEN(anon_sym_compute);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(251);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 147},
  [2] = {.lex_state = 5},
  [3] = {.lex_state = 5},
  [4] = {.lex_state = 5},
  [5] = {.lex_state = 5},
  [6] = {.lex_state = 5},
  [7] = {.lex_state = 5},
  [8] = {.lex_state = 5},
  [9] = {.lex_state = 5},
  [10] = {.lex_state = 5},
  [11] = {.lex_state = 146},
  [12] = {.lex_state = 5},
  [13] = {.lex_state = 6},
  [14] = {.lex_state = 5},
  [15] = {.lex_state = 5},
  [16] = {.lex_state = 5},
  [17] = {.lex_state = 5},
  [18] = {.lex_state = 6},
  [19] = {.lex_state = 6},
  [20] = {.lex_state = 6},
  [21] = {.lex_state = 6},
  [22] = {.lex_state = 6},
  [23] = {.lex_state = 6},
  [24] = {.lex_state = 6},
  [25] = {.lex_state = 6},
  [26] = {.lex_state = 6},
  [27] = {.lex_state = 1},
  [28] = {.lex_state = 1},
  [29] = {.lex_state = 6},
  [30] = {.lex_state = 1},
  [31] = {.lex_state = 1},
  [32] = {.lex_state = 6},
  [33] = {.lex_state = 1},
  [34] = {.lex_state = 1},
  [35] = {.lex_state = 1},
  [36] = {.lex_state = 1},
  [37] = {.lex_state = 1},
  [38] = {.lex_state = 6},
  [39] = {.lex_state = 1},
  [40] = {.lex_state = 6},
  [41] = {.lex_state = 1},
  [42] = {.lex_state = 6},
  [43] = {.lex_state = 1},
  [44] = {.lex_state = 1},
  [45] = {.lex_state = 1},
  [46] = {.lex_state = 1},
  [47] = {.lex_state = 1},
  [48] = {.lex_state = 1},
  [49] = {.lex_state = 1},
  [50] = {.lex_state = 1},
  [51] = {.lex_state = 1},
  [52] = {.lex_state = 2},
  [53] = {.lex_state = 6},
  [54] = {.lex_state = 6},
  [55] = {.lex_state = 147},
  [56] = {.lex_state = 147},
  [57] = {.lex_state = 147},
  [58] = {.lex_state = 3},
  [59] = {.lex_state = 3},
  [60] = {.lex_state = 3},
  [61] = {.lex_state = 3},
  [62] = {.lex_state = 3},
  [63] = {.lex_state = 3},
  [64] = {.lex_state = 3},
  [65] = {.lex_state = 3},
  [66] = {.lex_state = 3},
  [67] = {.lex_state = 3},
  [68] = {.lex_state = 3},
  [69] = {.lex_state = 3},
  [70] = {.lex_state = 147},
  [71] = {.lex_state = 147},
  [72] = {.lex_state = 147},
  [73] = {.lex_state = 147},
  [74] = {.lex_state = 147},
  [75] = {.lex_state = 147},
  [76] = {.lex_state = 147},
  [77] = {.lex_state = 147},
  [78] = {.lex_state = 147},
  [79] = {.lex_state = 147},
  [80] = {.lex_state = 147},
  [81] = {.lex_state = 147},
  [82] = {.lex_state = 147},
  [83] = {.lex_state = 4},
  [84] = {.lex_state = 4},
  [85] = {.lex_state = 147},
  [86] = {.lex_state = 147},
  [87] = {.lex_state = 4},
  [88] = {.lex_state = 4},
  [89] = {.lex_state = 147},
  [90] = {.lex_state = 4},
  [91] = {.lex_state = 4},
  [92] = {.lex_state = 4},
  [93] = {.lex_state = 4},
  [94] = {.lex_state = 4},
  [95] = {.lex_state = 4},
  [96] = {.lex_state = 4},
  [97] = {.lex_state = 4},
  [98] = {.lex_state = 4},
  [99] = {.lex_state = 147},
  [100] = {.lex_state = 147},
  [101] = {.lex_state = 147},
  [102] = {.lex_state = 3},
  [103] = {.lex_state = 11},
  [104] = {.lex_state = 3},
  [105] = {.lex_state = 3},
  [106] = {.lex_state = 3},
  [107] = {.lex_state = 3},
  [108] = {.lex_state = 3},
  [109] = {.lex_state = 3},
  [110] = {.lex_state = 3},
  [111] = {.lex_state = 3},
  [112] = {.lex_state = 3},
  [113] = {.lex_state = 147},
  [114] = {.lex_state = 147},
  [115] = {.lex_state = 148},
  [116] = {.lex_state = 147},
  [117] = {.lex_state = 147},
  [118] = {.lex_state = 147},
  [119] = {.lex_state = 147},
  [120] = {.lex_state = 147},
  [121] = {.lex_state = 147},
  [122] = {.lex_state = 147},
  [123] = {.lex_state = 2},
  [124] = {.lex_state = 14},
  [125] = {.lex_state = 147},
  [126] = {.lex_state = 2},
  [127] = {.lex_state = 13},
  [128] = {.lex_state = 147},
  [129] = {.lex_state = 147},
  [130] = {.lex_state = 147},
  [131] = {.lex_state = 147},
  [132] = {.lex_state = 147},
  [133] = {.lex_state = 147},
  [134] = {.lex_state = 147},
  [135] = {.lex_state = 0},
  [136] = {.lex_state = 147},
  [137] = {.lex_state = 147},
  [138] = {.lex_state = 14},
  [139] = {.lex_state = 147},
  [140] = {.lex_state = 147},
  [141] = {.lex_state = 147},
  [142] = {.lex_state = 147},
  [143] = {.lex_state = 13},
  [144] = {.lex_state = 2},
  [145] = {.lex_state = 2},
  [146] = {.lex_state = 5},
  [147] = {.lex_state = 13},
  [148] = {.lex_state = 5},
  [149] = {.lex_state = 0},
  [150] = {.lex_state = 0},
  [151] = {.lex_state = 0},
  [152] = {.lex_state = 0},
  [153] = {.lex_state = 16},
  [154] = {.lex_state = 0},
  [155] = {.lex_state = 0},
  [156] = {.lex_state = 0},
  [157] = {.lex_state = 0},
  [158] = {.lex_state = 0},
  [159] = {.lex_state = 0},
  [160] = {.lex_state = 0},
  [161] = {.lex_state = 147},
  [162] = {.lex_state = 0},
  [163] = {.lex_state = 0},
  [164] = {.lex_state = 14},
  [165] = {.lex_state = 0},
  [166] = {.lex_state = 0},
  [167] = {.lex_state = 14},
  [168] = {.lex_state = 0},
  [169] = {.lex_state = 0},
  [170] = {.lex_state = 0},
  [171] = {.lex_state = 14},
  [172] = {.lex_state = 0},
  [173] = {.lex_state = 0},
  [174] = {.lex_state = 0},
  [175] = {.lex_state = 0},
  [176] = {.lex_state = 14},
  [177] = {.lex_state = 0},
  [178] = {.lex_state = 0},
  [179] = {.lex_state = 0},
  [180] = {.lex_state = 0},
  [181] = {.lex_state = 0},
  [182] = {.lex_state = 0},
  [183] = {.lex_state = 2},
  [184] = {.lex_state = 14},
  [185] = {.lex_state = 0},
  [186] = {.lex_state = 0},
  [187] = {.lex_state = 0},
  [188] = {.lex_state = 0},
  [189] = {.lex_state = 13},
  [190] = {.lex_state = 0},
  [191] = {.lex_state = 15},
  [192] = {.lex_state = 13},
  [193] = {.lex_state = 2},
  [194] = {.lex_state = 0},
  [195] = {.lex_state = 5},
  [196] = {.lex_state = 13},
  [197] = {.lex_state = 0},
  [198] = {.lex_state = 2},
  [199] = {.lex_state = 0},
  [200] = {.lex_state = 0},
  [201] = {.lex_state = 14},
  [202] = {.lex_state = 0},
  [203] = {.lex_state = 0},
  [204] = {.lex_state = 15},
  [205] = {.lex_state = 14},
  [206] = {.lex_state = 14},
  [207] = {.lex_state = 0},
  [208] = {.lex_state = 0},
  [209] = {.lex_state = 0},
  [210] = {.lex_state = 0},
  [211] = {.lex_state = 0},
  [212] = {.lex_state = 0},
  [213] = {.lex_state = 14},
  [214] = {.lex_state = 14},
  [215] = {.lex_state = 0},
  [216] = {.lex_state = 14},
  [217] = {.lex_state = 14},
  [218] = {.lex_state = 0},
  [219] = {.lex_state = 14},
  [220] = {.lex_state = 5},
  [221] = {.lex_state = 147},
  [222] = {.lex_state = 14},
  [223] = {.lex_state = 14},
  [224] = {.lex_state = 0},
  [225] = {.lex_state = 0},
  [226] = {.lex_state = 14},
  [227] = {.lex_state = 0},
  [228] = {.lex_state = 0},
  [229] = {.lex_state = 14},
  [230] = {.lex_state = 0},
  [231] = {.lex_state = 0},
  [232] = {.lex_state = 0},
  [233] = {.lex_state = 0},
  [234] = {.lex_state = 0},
  [235] = {.lex_state = 0},
  [236] = {.lex_state = 14},
  [237] = {.lex_state = 0},
  [238] = {.lex_state = 0},
  [239] = {.lex_state = 0},
  [240] = {.lex_state = 147},
  [241] = {.lex_state = 14},
  [242] = {.lex_state = 147},
  [243] = {.lex_state = 0},
  [244] = {.lex_state = 14},
  [245] = {.lex_state = 14},
  [246] = {.lex_state = 0},
  [247] = {.lex_state = 14},
  [248] = {.lex_state = 0},
  [249] = {.lex_state = 14},
  [250] = {.lex_state = 147},
  [251] = {.lex_state = 14},
  [252] = {.lex_state = 0},
  [253] = {.lex_state = 147},
  [254] = {.lex_state = 0},
  [255] = {.lex_state = 0},
  [256] = {.lex_state = 5},
  [257] = {.lex_state = 14},
  [258] = {.lex_state = 0},
  [259] = {.lex_state = 0},
  [260] = {.lex_state = 0},
  [261] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [sym_string] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_Uuid] = ACTIONS(1),
    [anon_sym_String] = ACTIONS(1),
    [anon_sym_Int] = ACTIONS(1),
    [anon_sym_Float] = ACTIONS(1),
    [anon_sym_Bool] = ACTIONS(1),
    [anon_sym_Date] = ACTIONS(1),
    [anon_sym_Timestamp] = ACTIONS(1),
    [anon_sym_Money] = ACTIONS(1),
    [anon_sym_Concrete] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_DOT_DOT] = ACTIONS(1),
    [anon_sym_AMP] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym__] = ACTIONS(1),
    [anon_sym_QMARK] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_main] = ACTIONS(1),
    [anon_sym_out] = ACTIONS(1),
    [anon_sym_source] = ACTIONS(1),
    [anon_sym_constraint] = ACTIONS(1),
    [anon_sym_doc] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_PIPE_PIPE] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_in] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_forall] = ACTIONS(1),
    [anon_sym_exists] = ACTIONS(1),
    [anon_sym_unique] = ACTIONS(1),
    [anon_sym_EQ_GT] = ACTIONS(1),
    [anon_sym_count] = ACTIONS(1),
    [anon_sym_templateVars] = ACTIONS(1),
    [anon_sym_keys] = ACTIONS(1),
    [anon_sym_type] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [sym_main_annotation] = ACTIONS(1),
    [anon_sym_ATdoc] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_as] = ACTIONS(1),
    [anon_sym_compute] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(261),
    [sym__definition] = STATE(56),
    [sym_annotation] = STATE(144),
    [sym_type_decl] = STATE(56),
    [sym_instance] = STATE(56),
    [sym_doc_annotation] = STATE(241),
    [sym_import_stmt] = STATE(56),
    [aux_sym_source_file_repeat1] = STATE(56),
    [aux_sym_field_repeat1] = STATE(144),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [sym_identifier] = ACTIONS(7),
    [anon_sym_AT] = ACTIONS(9),
    [anon_sym_type] = ACTIONS(11),
    [sym_main_annotation] = ACTIONS(13),
    [anon_sym_ATdoc] = ACTIONS(15),
    [anon_sym_import] = ACTIONS(17),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    ACTIONS(39), 1,
      anon_sym_QMARK,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(136), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [63] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    ACTIONS(41), 1,
      anon_sym_QMARK,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(140), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [126] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(113), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [186] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(114), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [246] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(161), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [306] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(139), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [366] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(76), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [426] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(75), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [486] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(141), 12,
      sym_base_type,
      sym__type_expr,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [546] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(43), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(54), 3,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    STATE(99), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [605] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    ACTIONS(58), 1,
      anon_sym_LBRACK,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(60), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(208), 9,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
  [666] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(62), 1,
      sym_identifier,
    ACTIONS(64), 1,
      anon_sym_RBRACK,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    STATE(175), 1,
      sym_list_element,
    STATE(258), 1,
      sym_list_elements,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(203), 8,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_refinement,
      sym_variant_value,
  [725] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(73), 9,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
  [782] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    ACTIONS(58), 1,
      anon_sym_LBRACK,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(80), 9,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
  [839] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    ACTIONS(58), 1,
      anon_sym_LBRACK,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(73), 9,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
  [896] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(29), 1,
      anon_sym_Concrete,
    ACTIONS(31), 1,
      anon_sym_LBRACK,
    ACTIONS(33), 1,
      anon_sym_AMP,
    ACTIONS(35), 1,
      anon_sym_DASH,
    ACTIONS(37), 1,
      anon_sym_LBRACE,
    STATE(89), 1,
      sym_boolean,
    ACTIONS(21), 2,
      sym_string,
      sym_number,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(80), 9,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_refinable_ref,
      sym_struct_type,
  [953] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(66), 1,
      sym_identifier,
    ACTIONS(68), 1,
      anon_sym_AMP,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(43), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(99), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1009] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(70), 1,
      sym_identifier,
    ACTIONS(72), 1,
      anon_sym_STAR,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(43), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
    STATE(99), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1063] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(75), 1,
      sym_identifier,
    ACTIONS(77), 1,
      anon_sym_LT,
    STATE(29), 1,
      sym_assoc_list,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    STATE(118), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1118] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(75), 1,
      sym_identifier,
    ACTIONS(77), 1,
      anon_sym_LT,
    STATE(32), 1,
      sym_assoc_list,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    STATE(116), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1173] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(75), 1,
      sym_identifier,
    ACTIONS(77), 1,
      anon_sym_LT,
    STATE(40), 1,
      sym_assoc_list,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    STATE(119), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1228] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(66), 1,
      sym_identifier,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(43), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(99), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1281] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(62), 1,
      sym_identifier,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    STATE(185), 1,
      sym_list_element,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(203), 8,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_refinement,
      sym_variant_value,
  [1334] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(70), 1,
      sym_identifier,
    ACTIONS(79), 1,
      anon_sym_QMARK,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(128), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1386] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(70), 1,
      sym_identifier,
    ACTIONS(81), 1,
      anon_sym_QMARK,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(129), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1438] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(69), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1485] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(112), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1532] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(75), 1,
      sym_identifier,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    STATE(121), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1581] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(61), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1628] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(110), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1675] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(75), 1,
      sym_identifier,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    STATE(122), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1724] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(108), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1771] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(104), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1818] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(62), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1865] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(59), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1912] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(60), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [1959] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(70), 1,
      sym_identifier,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(130), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [2008] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(106), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2055] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(75), 1,
      sym_identifier,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(23), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    STATE(117), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [2104] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 1,
      sym_identifier,
    ACTIONS(101), 1,
      anon_sym_BANG,
    ACTIONS(103), 1,
      anon_sym_LPAREN,
    STATE(84), 1,
      sym_boolean,
    ACTIONS(97), 2,
      sym_string,
      sym_number,
    ACTIONS(99), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(105), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(107), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(87), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2151] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(25), 1,
      anon_sym_STAR,
    ACTIONS(50), 1,
      anon_sym_LBRACK,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    ACTIONS(70), 1,
      sym_identifier,
    STATE(72), 1,
      sym_boolean,
    STATE(78), 1,
      sym_base_type,
    ACTIONS(48), 2,
      sym_string,
      sym_number,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    STATE(133), 7,
      sym__value,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(27), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [2200] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(105), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2247] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 1,
      sym_identifier,
    ACTIONS(101), 1,
      anon_sym_BANG,
    ACTIONS(103), 1,
      anon_sym_LPAREN,
    STATE(84), 1,
      sym_boolean,
    ACTIONS(97), 2,
      sym_string,
      sym_number,
    ACTIONS(99), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(105), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(107), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(88), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2294] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 1,
      sym_identifier,
    ACTIONS(101), 1,
      anon_sym_BANG,
    ACTIONS(103), 1,
      anon_sym_LPAREN,
    STATE(84), 1,
      sym_boolean,
    ACTIONS(97), 2,
      sym_string,
      sym_number,
    ACTIONS(99), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(105), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(107), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(91), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2341] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 1,
      sym_identifier,
    ACTIONS(101), 1,
      anon_sym_BANG,
    ACTIONS(103), 1,
      anon_sym_LPAREN,
    STATE(84), 1,
      sym_boolean,
    ACTIONS(97), 2,
      sym_string,
      sym_number,
    ACTIONS(99), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(105), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(107), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(93), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2388] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(95), 1,
      sym_identifier,
    ACTIONS(101), 1,
      anon_sym_BANG,
    ACTIONS(103), 1,
      anon_sym_LPAREN,
    STATE(84), 1,
      sym_boolean,
    ACTIONS(97), 2,
      sym_string,
      sym_number,
    ACTIONS(99), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(105), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(107), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(94), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2435] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(109), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2482] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(102), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2529] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(107), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2576] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(83), 1,
      sym_identifier,
    ACTIONS(87), 1,
      anon_sym_BANG,
    ACTIONS(89), 1,
      anon_sym_LPAREN,
    STATE(65), 1,
      sym_boolean,
    ACTIONS(56), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(85), 2,
      sym_string,
      sym_number,
    ACTIONS(91), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(93), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(111), 11,
      sym__constraint_expr,
      sym_constraint_or,
      sym_constraint_and,
      sym_constraint_not,
      sym_constraint_comparison,
      sym__constraint_primary,
      sym_constraint_paren,
      sym_constraint_call,
      sym_constraint_field_access,
      sym_constraint_var,
      sym_constraint_literal,
  [2623] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(111), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
    ACTIONS(109), 15,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [2649] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(115), 5,
      sym_string,
      sym_number,
      anon_sym_STAR,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
    ACTIONS(113), 11,
      sym_identifier,
      anon_sym_true,
      anon_sym_false,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [2673] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(119), 5,
      sym_string,
      sym_number,
      anon_sym_STAR,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
    ACTIONS(117), 11,
      sym_identifier,
      anon_sym_true,
      anon_sym_false,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [2697] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(123), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(121), 11,
      ts_builtin_sym_end,
      anon_sym_STAR,
      anon_sym_GT,
      anon_sym_RBRACK,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_EQ,
      sym_main_annotation,
      anon_sym_ATdoc,
  [2720] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(9), 1,
      anon_sym_AT,
    ACTIONS(11), 1,
      anon_sym_type,
    ACTIONS(13), 1,
      sym_main_annotation,
    ACTIONS(15), 1,
      anon_sym_ATdoc,
    ACTIONS(17), 1,
      anon_sym_import,
    ACTIONS(125), 1,
      ts_builtin_sym_end,
    STATE(241), 1,
      sym_doc_annotation,
    STATE(144), 2,
      sym_annotation,
      aux_sym_field_repeat1,
    STATE(57), 5,
      sym__definition,
      sym_type_decl,
      sym_instance,
      sym_import_stmt,
      aux_sym_source_file_repeat1,
  [2759] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(127), 1,
      ts_builtin_sym_end,
    ACTIONS(129), 1,
      sym_identifier,
    ACTIONS(132), 1,
      anon_sym_AT,
    ACTIONS(135), 1,
      anon_sym_type,
    ACTIONS(138), 1,
      sym_main_annotation,
    ACTIONS(141), 1,
      anon_sym_ATdoc,
    ACTIONS(144), 1,
      anon_sym_import,
    STATE(241), 1,
      sym_doc_annotation,
    STATE(144), 2,
      sym_annotation,
      aux_sym_field_repeat1,
    STATE(57), 5,
      sym__definition,
      sym_type_decl,
      sym_instance,
      sym_import_stmt,
      aux_sym_source_file_repeat1,
  [2798] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(147), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(149), 12,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [2820] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(153), 5,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
      anon_sym_type,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [2848] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
    ACTIONS(161), 6,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_RPAREN,
      anon_sym_type,
  [2874] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(163), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(165), 11,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [2898] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(167), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(169), 11,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [2922] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(173), 12,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [2944] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(177), 12,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [2966] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(179), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(181), 12,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [2988] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(183), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(185), 12,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [3010] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(189), 12,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [3032] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(193), 12,
      anon_sym_COMMA,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
      anon_sym_type,
  [3054] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(195), 2,
      anon_sym_AT,
      anon_sym_type,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3082] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(199), 8,
      ts_builtin_sym_end,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3102] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(203), 8,
      ts_builtin_sym_end,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3122] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(207), 8,
      ts_builtin_sym_end,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3142] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(213), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(211), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3162] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(215), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3182] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(221), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(219), 7,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3204] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(227), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(225), 7,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3226] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(231), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(229), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3246] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(233), 8,
      ts_builtin_sym_end,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3266] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(239), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(237), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3286] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(241), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3306] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(247), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(245), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3326] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(251), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(249), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3346] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(171), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(173), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3366] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(179), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(181), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3386] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(111), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(109), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3406] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(255), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(253), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3426] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(195), 1,
      anon_sym_AT,
    ACTIONS(257), 1,
      sym_identifier,
    ACTIONS(261), 1,
      anon_sym_DOT,
    ACTIONS(263), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(265), 1,
      anon_sym_AMP_AMP,
    ACTIONS(259), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(267), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3456] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(261), 1,
      anon_sym_DOT,
    ACTIONS(163), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(165), 7,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3478] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(271), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(269), 8,
      ts_builtin_sym_end,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3498] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(191), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(193), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3518] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(261), 1,
      anon_sym_DOT,
    ACTIONS(167), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(169), 7,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3540] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(147), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(149), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3560] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(261), 1,
      anon_sym_DOT,
    ACTIONS(265), 1,
      anon_sym_AMP_AMP,
    ACTIONS(273), 1,
      sym_identifier,
    ACTIONS(153), 2,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
    ACTIONS(259), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(267), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3588] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(261), 1,
      anon_sym_DOT,
    ACTIONS(275), 1,
      sym_identifier,
    ACTIONS(161), 3,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
    ACTIONS(259), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(267), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3614] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(177), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3634] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(183), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(185), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3654] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(187), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(189), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3674] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(111), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(109), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3694] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(279), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(277), 8,
      ts_builtin_sym_end,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3714] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(283), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(281), 8,
      ts_builtin_sym_end,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3734] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(287), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
    ACTIONS(285), 8,
      ts_builtin_sym_end,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
      sym_main_annotation,
      anon_sym_ATdoc,
  [3754] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(289), 1,
      anon_sym_RPAREN,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3781] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(291), 1,
      sym_identifier,
    ACTIONS(293), 1,
      anon_sym_DOT_DOT_DOT,
    ACTIONS(295), 1,
      anon_sym_RBRACE,
    ACTIONS(297), 1,
      anon_sym__,
    ACTIONS(299), 1,
      anon_sym_AT,
    STATE(173), 1,
      sym_anonymous_field,
    STATE(174), 1,
      sym_field,
    STATE(147), 2,
      sym_annotation,
      aux_sym_field_repeat1,
    STATE(230), 2,
      sym_anonymous_fields,
      sym_field_list,
  [3814] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(301), 1,
      anon_sym_RPAREN,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3841] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(303), 1,
      anon_sym_RPAREN,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3868] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(305), 1,
      anon_sym_RPAREN,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3895] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(307), 1,
      anon_sym_RPAREN,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3922] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(309), 1,
      anon_sym_COMMA,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3949] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(311), 1,
      anon_sym_RPAREN,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3976] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(313), 1,
      anon_sym_RPAREN,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [4003] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(315), 1,
      anon_sym_RPAREN,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [4030] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(155), 1,
      anon_sym_DOT,
    ACTIONS(157), 1,
      anon_sym_AMP_AMP,
    ACTIONS(197), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(317), 1,
      anon_sym_COMMA,
    ACTIONS(151), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [4057] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(323), 1,
      anon_sym_PIPE,
    ACTIONS(319), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(321), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4078] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(323), 1,
      anon_sym_PIPE,
    ACTIONS(325), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(327), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4099] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(333), 1,
      anon_sym_as,
    ACTIONS(329), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(331), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4117] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(335), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(337), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4132] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(339), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(341), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4147] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(343), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(345), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4162] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(347), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(349), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4177] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(351), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(353), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4192] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(355), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(357), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4207] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(359), 3,
      ts_builtin_sym_end,
      sym_main_annotation,
      anon_sym_ATdoc,
    ACTIONS(361), 4,
      sym_identifier,
      anon_sym_AT,
      anon_sym_type,
      anon_sym_import,
  [4222] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(365), 1,
      anon_sym_assoc,
    ACTIONS(367), 1,
      anon_sym_source,
    ACTIONS(369), 1,
      anon_sym_constraint,
    ACTIONS(371), 1,
      anon_sym_doc,
    ACTIONS(363), 2,
      anon_sym_main,
      anon_sym_out,
  [4242] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_ATdoc,
    ACTIONS(373), 1,
      sym_identifier,
    ACTIONS(375), 1,
      anon_sym_RBRACE,
    STATE(154), 1,
      sym_value_field,
    STATE(229), 1,
      sym_doc_annotation,
    STATE(231), 1,
      sym_value_field_list,
  [4264] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(379), 1,
      anon_sym_DOT,
    STATE(125), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(377), 4,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [4280] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(384), 1,
      anon_sym_assoc,
    ACTIONS(386), 1,
      anon_sym_source,
    ACTIONS(388), 1,
      anon_sym_constraint,
    ACTIONS(390), 1,
      anon_sym_doc,
    ACTIONS(382), 2,
      anon_sym_main,
      anon_sym_out,
  [4300] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(299), 1,
      anon_sym_AT,
    ACTIONS(392), 1,
      sym_identifier,
    STATE(181), 1,
      sym_field,
    STATE(147), 2,
      sym_annotation,
      aux_sym_field_repeat1,
  [4317] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(394), 1,
      anon_sym_STAR,
    ACTIONS(398), 1,
      anon_sym_EQ,
    STATE(194), 1,
      sym_field_origin,
    ACTIONS(396), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4334] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(394), 1,
      anon_sym_STAR,
    ACTIONS(398), 1,
      anon_sym_EQ,
    STATE(200), 1,
      sym_field_origin,
    ACTIONS(400), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4351] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(394), 1,
      anon_sym_STAR,
    ACTIONS(398), 1,
      anon_sym_EQ,
    STATE(188), 1,
      sym_field_origin,
    ACTIONS(402), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4368] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(377), 5,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_RPAREN,
  [4379] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(406), 1,
      anon_sym_DOT,
    STATE(134), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(404), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [4394] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(394), 1,
      anon_sym_STAR,
    ACTIONS(398), 1,
      anon_sym_EQ,
    STATE(180), 1,
      sym_field_origin,
    ACTIONS(408), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4411] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(406), 1,
      anon_sym_DOT,
    STATE(125), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(410), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [4426] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(414), 1,
      anon_sym_COMMA,
    STATE(135), 1,
      aux_sym_annotation_args_repeat1,
    ACTIONS(412), 2,
      anon_sym_GT,
      anon_sym_RBRACK,
  [4440] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(323), 1,
      anon_sym_PIPE,
    ACTIONS(417), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4454] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(406), 1,
      anon_sym_DOT,
    STATE(142), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(419), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [4468] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_ATdoc,
    ACTIONS(373), 1,
      sym_identifier,
    STATE(182), 1,
      sym_value_field,
    STATE(229), 1,
      sym_doc_annotation,
  [4484] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(323), 1,
      anon_sym_PIPE,
    ACTIONS(421), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4498] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(323), 1,
      anon_sym_PIPE,
    ACTIONS(423), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4512] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(323), 1,
      anon_sym_PIPE,
    ACTIONS(425), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4526] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(406), 1,
      anon_sym_DOT,
    STATE(125), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(427), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [4540] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(429), 1,
      sym_identifier,
    ACTIONS(431), 1,
      anon_sym_AT,
    STATE(143), 2,
      sym_annotation,
      aux_sym_field_repeat1,
  [4554] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(434), 1,
      anon_sym_AT,
    ACTIONS(436), 1,
      anon_sym_type,
    STATE(145), 2,
      sym_annotation,
      aux_sym_field_repeat1,
  [4568] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(429), 1,
      anon_sym_type,
    ACTIONS(438), 1,
      anon_sym_AT,
    STATE(145), 2,
      sym_annotation,
      aux_sym_field_repeat1,
  [4582] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_number,
    ACTIONS(443), 1,
      anon_sym_RBRACK,
    ACTIONS(445), 1,
      anon_sym_DOT_DOT,
    STATE(225), 1,
      sym_cardinality,
  [4598] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(299), 1,
      anon_sym_AT,
    ACTIONS(447), 1,
      sym_identifier,
    STATE(143), 2,
      sym_annotation,
      aux_sym_field_repeat1,
  [4612] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(441), 1,
      sym_number,
    ACTIONS(445), 1,
      anon_sym_DOT_DOT,
    ACTIONS(449), 1,
      anon_sym_RBRACK,
    STATE(243), 1,
      sym_cardinality,
  [4628] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(451), 1,
      anon_sym_RBRACE,
    ACTIONS(453), 1,
      anon_sym_COMMA,
    STATE(155), 1,
      aux_sym_value_field_list_repeat1,
  [4641] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(455), 1,
      anon_sym_GT,
    ACTIONS(457), 1,
      anon_sym_COMMA,
    STATE(135), 1,
      aux_sym_annotation_args_repeat1,
  [4654] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(459), 1,
      anon_sym_RBRACK,
    ACTIONS(461), 1,
      anon_sym_COMMA,
    STATE(151), 1,
      aux_sym_list_elements_repeat1,
  [4667] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(464), 1,
      anon_sym_RBRACK,
    ACTIONS(466), 1,
      anon_sym_COMMA,
    STATE(151), 1,
      aux_sym_list_elements_repeat1,
  [4680] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(468), 1,
      sym_identifier,
    ACTIONS(470), 1,
      anon_sym_compute,
    STATE(197), 1,
      sym_origin_path,
  [4693] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(453), 1,
      anon_sym_COMMA,
    ACTIONS(472), 1,
      anon_sym_RBRACE,
    STATE(149), 1,
      aux_sym_value_field_list_repeat1,
  [4706] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(474), 1,
      anon_sym_RBRACE,
    ACTIONS(476), 1,
      anon_sym_COMMA,
    STATE(155), 1,
      aux_sym_value_field_list_repeat1,
  [4719] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(457), 1,
      anon_sym_COMMA,
    ACTIONS(479), 1,
      anon_sym_RBRACK,
    STATE(135), 1,
      aux_sym_annotation_args_repeat1,
  [4732] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(481), 1,
      anon_sym_RBRACK,
    ACTIONS(483), 1,
      anon_sym_COMMA,
    STATE(157), 1,
      aux_sym_source_args_repeat1,
  [4745] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(486), 1,
      anon_sym_RBRACE,
    ACTIONS(488), 1,
      anon_sym_COMMA,
    STATE(158), 1,
      aux_sym_anonymous_fields_repeat1,
  [4758] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(491), 1,
      anon_sym_RBRACE,
    ACTIONS(493), 1,
      anon_sym_COMMA,
    STATE(159), 1,
      aux_sym_field_list_repeat1,
  [4771] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(457), 1,
      anon_sym_COMMA,
    ACTIONS(496), 1,
      anon_sym_GT,
    STATE(150), 1,
      aux_sym_annotation_args_repeat1,
  [4784] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 1,
      anon_sym_AMP,
    ACTIONS(323), 1,
      anon_sym_PIPE,
    ACTIONS(498), 1,
      anon_sym_GT,
  [4797] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(412), 3,
      anon_sym_GT,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [4806] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(457), 1,
      anon_sym_COMMA,
    ACTIONS(500), 1,
      anon_sym_RBRACK,
    STATE(156), 1,
      aux_sym_annotation_args_repeat1,
  [4819] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(502), 1,
      sym_identifier,
    STATE(166), 1,
      sym_origin_path,
    STATE(235), 1,
      sym_origin_paths,
  [4832] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(504), 1,
      anon_sym_RBRACK,
    ACTIONS(506), 1,
      anon_sym_COMMA,
    STATE(157), 1,
      aux_sym_source_args_repeat1,
  [4845] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(508), 1,
      anon_sym_COMMA,
    ACTIONS(510), 1,
      anon_sym_RPAREN,
    STATE(168), 1,
      aux_sym_origin_paths_repeat1,
  [4858] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(15), 1,
      anon_sym_ATdoc,
    ACTIONS(512), 1,
      sym_identifier,
    STATE(251), 1,
      sym_doc_annotation,
  [4871] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(508), 1,
      anon_sym_COMMA,
    ACTIONS(514), 1,
      anon_sym_RPAREN,
    STATE(169), 1,
      aux_sym_origin_paths_repeat1,
  [4884] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(516), 1,
      anon_sym_COMMA,
    ACTIONS(519), 1,
      anon_sym_RPAREN,
    STATE(169), 1,
      aux_sym_origin_paths_repeat1,
  [4897] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(521), 1,
      anon_sym_RBRACE,
    ACTIONS(523), 1,
      anon_sym_COMMA,
    STATE(158), 1,
      aux_sym_anonymous_fields_repeat1,
  [4910] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      sym_identifier,
    STATE(177), 1,
      sym_source_path,
    STATE(233), 1,
      sym_source_args,
  [4923] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(527), 1,
      anon_sym_RBRACE,
    ACTIONS(529), 1,
      anon_sym_COMMA,
    STATE(159), 1,
      aux_sym_field_list_repeat1,
  [4936] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(523), 1,
      anon_sym_COMMA,
    ACTIONS(531), 1,
      anon_sym_RBRACE,
    STATE(170), 1,
      aux_sym_anonymous_fields_repeat1,
  [4949] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(529), 1,
      anon_sym_COMMA,
    ACTIONS(533), 1,
      anon_sym_RBRACE,
    STATE(172), 1,
      aux_sym_field_list_repeat1,
  [4962] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(466), 1,
      anon_sym_COMMA,
    ACTIONS(535), 1,
      anon_sym_RBRACK,
    STATE(152), 1,
      aux_sym_list_elements_repeat1,
  [4975] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      sym_identifier,
    STATE(177), 1,
      sym_source_path,
    STATE(238), 1,
      sym_source_args,
  [4988] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(506), 1,
      anon_sym_COMMA,
    ACTIONS(537), 1,
      anon_sym_RBRACK,
    STATE(165), 1,
      aux_sym_source_args_repeat1,
  [5001] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(539), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5009] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(481), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5017] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(541), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5025] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(491), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5033] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(474), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5041] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 2,
      anon_sym_AT,
      anon_sym_type,
  [5049] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(502), 1,
      sym_identifier,
    STATE(187), 1,
      sym_origin_path,
  [5059] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(459), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5067] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(545), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5075] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(519), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [5083] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(547), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5091] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(549), 2,
      sym_identifier,
      anon_sym_AT,
  [5099] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(551), 1,
      anon_sym__,
    STATE(207), 1,
      sym_anonymous_field,
  [5109] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(553), 1,
      sym_identifier,
    ACTIONS(555), 1,
      anon_sym_assoc,
  [5119] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(195), 2,
      sym_identifier,
      anon_sym_AT,
  [5127] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(195), 2,
      anon_sym_AT,
      anon_sym_type,
  [5135] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(557), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5143] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(559), 1,
      anon_sym_RBRACK,
    ACTIONS(561), 1,
      anon_sym_DOT_DOT,
  [5153] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(543), 2,
      sym_identifier,
      anon_sym_AT,
  [5161] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(563), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5169] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(549), 2,
      anon_sym_AT,
      anon_sym_type,
  [5177] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(565), 1,
      sym_number,
    ACTIONS(567), 1,
      anon_sym_RBRACK,
  [5187] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(569), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5195] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 1,
      sym_identifier,
    STATE(233), 1,
      sym_annotation_args,
  [5205] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(573), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5213] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(575), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5221] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(577), 1,
      sym_identifier,
    ACTIONS(579), 1,
      anon_sym_assoc,
  [5231] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(525), 1,
      sym_identifier,
    STATE(179), 1,
      sym_source_path,
  [5241] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(571), 1,
      sym_identifier,
    STATE(238), 1,
      sym_annotation_args,
  [5251] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(486), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5259] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(581), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5267] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(52), 1,
      anon_sym_LBRACE,
    STATE(178), 1,
      sym_struct_value,
  [5277] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(583), 1,
      anon_sym_LBRACK,
  [5284] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(585), 1,
      anon_sym_LPAREN,
  [5291] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(587), 1,
      anon_sym_LPAREN,
  [5298] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(589), 1,
      sym_identifier,
  [5305] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(591), 1,
      sym_identifier,
  [5312] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(593), 1,
      anon_sym_LPAREN,
  [5319] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(595), 1,
      sym_identifier,
  [5326] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(597), 1,
      sym_identifier,
  [5333] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(599), 1,
      sym_string,
  [5340] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(601), 1,
      sym_identifier,
  [5347] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(603), 1,
      anon_sym_EQ_GT,
  [5354] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(605), 1,
      anon_sym_EQ,
  [5361] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(607), 1,
      sym_identifier,
  [5368] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(609), 1,
      sym_identifier,
  [5375] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(611), 1,
      sym_number,
  [5382] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(613), 1,
      anon_sym_RBRACK,
  [5389] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(615), 1,
      sym_identifier,
  [5396] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(617), 1,
      anon_sym_LT,
  [5403] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(619), 1,
      anon_sym_RBRACK,
  [5410] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(621), 1,
      sym_identifier,
  [5417] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(623), 1,
      anon_sym_RBRACE,
  [5424] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(625), 1,
      anon_sym_RBRACE,
  [5431] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(627), 1,
      anon_sym_LBRACK,
  [5438] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(629), 1,
      anon_sym_RBRACK,
  [5445] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(631), 1,
      anon_sym_LPAREN,
  [5452] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(633), 1,
      anon_sym_RPAREN,
  [5459] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(635), 1,
      sym_identifier,
  [5466] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(637), 1,
      sym_string,
  [5473] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(639), 1,
      anon_sym_RBRACK,
  [5480] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(641), 1,
      anon_sym_LBRACK,
  [5487] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(643), 1,
      anon_sym_EQ,
  [5494] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(512), 1,
      sym_identifier,
  [5501] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(645), 1,
      anon_sym_EQ,
  [5508] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(647), 1,
      anon_sym_RBRACK,
  [5515] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(649), 1,
      sym_identifier,
  [5522] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(651), 1,
      sym_identifier,
  [5529] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(567), 1,
      anon_sym_RBRACK,
  [5536] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(653), 1,
      sym_identifier,
  [5543] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(655), 1,
      sym_string,
  [5550] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(657), 1,
      sym_identifier,
  [5557] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(659), 1,
      anon_sym_EQ,
  [5564] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(661), 1,
      sym_identifier,
  [5571] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(663), 1,
      anon_sym_LBRACK,
  [5578] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(665), 1,
      anon_sym_EQ,
  [5585] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(667), 1,
      anon_sym_LPAREN,
  [5592] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(669), 1,
      anon_sym_LPAREN,
  [5599] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(671), 1,
      anon_sym_EQ_GT,
  [5606] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(673), 1,
      sym_identifier,
  [5613] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(675), 1,
      anon_sym_RBRACK,
  [5620] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(677), 1,
      sym_string,
  [5627] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(679), 1,
      anon_sym_LPAREN,
  [5634] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(681), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 63,
  [SMALL_STATE(4)] = 126,
  [SMALL_STATE(5)] = 186,
  [SMALL_STATE(6)] = 246,
  [SMALL_STATE(7)] = 306,
  [SMALL_STATE(8)] = 366,
  [SMALL_STATE(9)] = 426,
  [SMALL_STATE(10)] = 486,
  [SMALL_STATE(11)] = 546,
  [SMALL_STATE(12)] = 605,
  [SMALL_STATE(13)] = 666,
  [SMALL_STATE(14)] = 725,
  [SMALL_STATE(15)] = 782,
  [SMALL_STATE(16)] = 839,
  [SMALL_STATE(17)] = 896,
  [SMALL_STATE(18)] = 953,
  [SMALL_STATE(19)] = 1009,
  [SMALL_STATE(20)] = 1063,
  [SMALL_STATE(21)] = 1118,
  [SMALL_STATE(22)] = 1173,
  [SMALL_STATE(23)] = 1228,
  [SMALL_STATE(24)] = 1281,
  [SMALL_STATE(25)] = 1334,
  [SMALL_STATE(26)] = 1386,
  [SMALL_STATE(27)] = 1438,
  [SMALL_STATE(28)] = 1485,
  [SMALL_STATE(29)] = 1532,
  [SMALL_STATE(30)] = 1581,
  [SMALL_STATE(31)] = 1628,
  [SMALL_STATE(32)] = 1675,
  [SMALL_STATE(33)] = 1724,
  [SMALL_STATE(34)] = 1771,
  [SMALL_STATE(35)] = 1818,
  [SMALL_STATE(36)] = 1865,
  [SMALL_STATE(37)] = 1912,
  [SMALL_STATE(38)] = 1959,
  [SMALL_STATE(39)] = 2008,
  [SMALL_STATE(40)] = 2055,
  [SMALL_STATE(41)] = 2104,
  [SMALL_STATE(42)] = 2151,
  [SMALL_STATE(43)] = 2200,
  [SMALL_STATE(44)] = 2247,
  [SMALL_STATE(45)] = 2294,
  [SMALL_STATE(46)] = 2341,
  [SMALL_STATE(47)] = 2388,
  [SMALL_STATE(48)] = 2435,
  [SMALL_STATE(49)] = 2482,
  [SMALL_STATE(50)] = 2529,
  [SMALL_STATE(51)] = 2576,
  [SMALL_STATE(52)] = 2623,
  [SMALL_STATE(53)] = 2649,
  [SMALL_STATE(54)] = 2673,
  [SMALL_STATE(55)] = 2697,
  [SMALL_STATE(56)] = 2720,
  [SMALL_STATE(57)] = 2759,
  [SMALL_STATE(58)] = 2798,
  [SMALL_STATE(59)] = 2820,
  [SMALL_STATE(60)] = 2848,
  [SMALL_STATE(61)] = 2874,
  [SMALL_STATE(62)] = 2898,
  [SMALL_STATE(63)] = 2922,
  [SMALL_STATE(64)] = 2944,
  [SMALL_STATE(65)] = 2966,
  [SMALL_STATE(66)] = 2988,
  [SMALL_STATE(67)] = 3010,
  [SMALL_STATE(68)] = 3032,
  [SMALL_STATE(69)] = 3054,
  [SMALL_STATE(70)] = 3082,
  [SMALL_STATE(71)] = 3102,
  [SMALL_STATE(72)] = 3122,
  [SMALL_STATE(73)] = 3142,
  [SMALL_STATE(74)] = 3162,
  [SMALL_STATE(75)] = 3182,
  [SMALL_STATE(76)] = 3204,
  [SMALL_STATE(77)] = 3226,
  [SMALL_STATE(78)] = 3246,
  [SMALL_STATE(79)] = 3266,
  [SMALL_STATE(80)] = 3286,
  [SMALL_STATE(81)] = 3306,
  [SMALL_STATE(82)] = 3326,
  [SMALL_STATE(83)] = 3346,
  [SMALL_STATE(84)] = 3366,
  [SMALL_STATE(85)] = 3386,
  [SMALL_STATE(86)] = 3406,
  [SMALL_STATE(87)] = 3426,
  [SMALL_STATE(88)] = 3456,
  [SMALL_STATE(89)] = 3478,
  [SMALL_STATE(90)] = 3498,
  [SMALL_STATE(91)] = 3518,
  [SMALL_STATE(92)] = 3540,
  [SMALL_STATE(93)] = 3560,
  [SMALL_STATE(94)] = 3588,
  [SMALL_STATE(95)] = 3614,
  [SMALL_STATE(96)] = 3634,
  [SMALL_STATE(97)] = 3654,
  [SMALL_STATE(98)] = 3674,
  [SMALL_STATE(99)] = 3694,
  [SMALL_STATE(100)] = 3714,
  [SMALL_STATE(101)] = 3734,
  [SMALL_STATE(102)] = 3754,
  [SMALL_STATE(103)] = 3781,
  [SMALL_STATE(104)] = 3814,
  [SMALL_STATE(105)] = 3841,
  [SMALL_STATE(106)] = 3868,
  [SMALL_STATE(107)] = 3895,
  [SMALL_STATE(108)] = 3922,
  [SMALL_STATE(109)] = 3949,
  [SMALL_STATE(110)] = 3976,
  [SMALL_STATE(111)] = 4003,
  [SMALL_STATE(112)] = 4030,
  [SMALL_STATE(113)] = 4057,
  [SMALL_STATE(114)] = 4078,
  [SMALL_STATE(115)] = 4099,
  [SMALL_STATE(116)] = 4117,
  [SMALL_STATE(117)] = 4132,
  [SMALL_STATE(118)] = 4147,
  [SMALL_STATE(119)] = 4162,
  [SMALL_STATE(120)] = 4177,
  [SMALL_STATE(121)] = 4192,
  [SMALL_STATE(122)] = 4207,
  [SMALL_STATE(123)] = 4222,
  [SMALL_STATE(124)] = 4242,
  [SMALL_STATE(125)] = 4264,
  [SMALL_STATE(126)] = 4280,
  [SMALL_STATE(127)] = 4300,
  [SMALL_STATE(128)] = 4317,
  [SMALL_STATE(129)] = 4334,
  [SMALL_STATE(130)] = 4351,
  [SMALL_STATE(131)] = 4368,
  [SMALL_STATE(132)] = 4379,
  [SMALL_STATE(133)] = 4394,
  [SMALL_STATE(134)] = 4411,
  [SMALL_STATE(135)] = 4426,
  [SMALL_STATE(136)] = 4440,
  [SMALL_STATE(137)] = 4454,
  [SMALL_STATE(138)] = 4468,
  [SMALL_STATE(139)] = 4484,
  [SMALL_STATE(140)] = 4498,
  [SMALL_STATE(141)] = 4512,
  [SMALL_STATE(142)] = 4526,
  [SMALL_STATE(143)] = 4540,
  [SMALL_STATE(144)] = 4554,
  [SMALL_STATE(145)] = 4568,
  [SMALL_STATE(146)] = 4582,
  [SMALL_STATE(147)] = 4598,
  [SMALL_STATE(148)] = 4612,
  [SMALL_STATE(149)] = 4628,
  [SMALL_STATE(150)] = 4641,
  [SMALL_STATE(151)] = 4654,
  [SMALL_STATE(152)] = 4667,
  [SMALL_STATE(153)] = 4680,
  [SMALL_STATE(154)] = 4693,
  [SMALL_STATE(155)] = 4706,
  [SMALL_STATE(156)] = 4719,
  [SMALL_STATE(157)] = 4732,
  [SMALL_STATE(158)] = 4745,
  [SMALL_STATE(159)] = 4758,
  [SMALL_STATE(160)] = 4771,
  [SMALL_STATE(161)] = 4784,
  [SMALL_STATE(162)] = 4797,
  [SMALL_STATE(163)] = 4806,
  [SMALL_STATE(164)] = 4819,
  [SMALL_STATE(165)] = 4832,
  [SMALL_STATE(166)] = 4845,
  [SMALL_STATE(167)] = 4858,
  [SMALL_STATE(168)] = 4871,
  [SMALL_STATE(169)] = 4884,
  [SMALL_STATE(170)] = 4897,
  [SMALL_STATE(171)] = 4910,
  [SMALL_STATE(172)] = 4923,
  [SMALL_STATE(173)] = 4936,
  [SMALL_STATE(174)] = 4949,
  [SMALL_STATE(175)] = 4962,
  [SMALL_STATE(176)] = 4975,
  [SMALL_STATE(177)] = 4988,
  [SMALL_STATE(178)] = 5001,
  [SMALL_STATE(179)] = 5009,
  [SMALL_STATE(180)] = 5017,
  [SMALL_STATE(181)] = 5025,
  [SMALL_STATE(182)] = 5033,
  [SMALL_STATE(183)] = 5041,
  [SMALL_STATE(184)] = 5049,
  [SMALL_STATE(185)] = 5059,
  [SMALL_STATE(186)] = 5067,
  [SMALL_STATE(187)] = 5075,
  [SMALL_STATE(188)] = 5083,
  [SMALL_STATE(189)] = 5091,
  [SMALL_STATE(190)] = 5099,
  [SMALL_STATE(191)] = 5109,
  [SMALL_STATE(192)] = 5119,
  [SMALL_STATE(193)] = 5127,
  [SMALL_STATE(194)] = 5135,
  [SMALL_STATE(195)] = 5143,
  [SMALL_STATE(196)] = 5153,
  [SMALL_STATE(197)] = 5161,
  [SMALL_STATE(198)] = 5169,
  [SMALL_STATE(199)] = 5177,
  [SMALL_STATE(200)] = 5187,
  [SMALL_STATE(201)] = 5195,
  [SMALL_STATE(202)] = 5205,
  [SMALL_STATE(203)] = 5213,
  [SMALL_STATE(204)] = 5221,
  [SMALL_STATE(205)] = 5231,
  [SMALL_STATE(206)] = 5241,
  [SMALL_STATE(207)] = 5251,
  [SMALL_STATE(208)] = 5259,
  [SMALL_STATE(209)] = 5267,
  [SMALL_STATE(210)] = 5277,
  [SMALL_STATE(211)] = 5284,
  [SMALL_STATE(212)] = 5291,
  [SMALL_STATE(213)] = 5298,
  [SMALL_STATE(214)] = 5305,
  [SMALL_STATE(215)] = 5312,
  [SMALL_STATE(216)] = 5319,
  [SMALL_STATE(217)] = 5326,
  [SMALL_STATE(218)] = 5333,
  [SMALL_STATE(219)] = 5340,
  [SMALL_STATE(220)] = 5347,
  [SMALL_STATE(221)] = 5354,
  [SMALL_STATE(222)] = 5361,
  [SMALL_STATE(223)] = 5368,
  [SMALL_STATE(224)] = 5375,
  [SMALL_STATE(225)] = 5382,
  [SMALL_STATE(226)] = 5389,
  [SMALL_STATE(227)] = 5396,
  [SMALL_STATE(228)] = 5403,
  [SMALL_STATE(229)] = 5410,
  [SMALL_STATE(230)] = 5417,
  [SMALL_STATE(231)] = 5424,
  [SMALL_STATE(232)] = 5431,
  [SMALL_STATE(233)] = 5438,
  [SMALL_STATE(234)] = 5445,
  [SMALL_STATE(235)] = 5452,
  [SMALL_STATE(236)] = 5459,
  [SMALL_STATE(237)] = 5466,
  [SMALL_STATE(238)] = 5473,
  [SMALL_STATE(239)] = 5480,
  [SMALL_STATE(240)] = 5487,
  [SMALL_STATE(241)] = 5494,
  [SMALL_STATE(242)] = 5501,
  [SMALL_STATE(243)] = 5508,
  [SMALL_STATE(244)] = 5515,
  [SMALL_STATE(245)] = 5522,
  [SMALL_STATE(246)] = 5529,
  [SMALL_STATE(247)] = 5536,
  [SMALL_STATE(248)] = 5543,
  [SMALL_STATE(249)] = 5550,
  [SMALL_STATE(250)] = 5557,
  [SMALL_STATE(251)] = 5564,
  [SMALL_STATE(252)] = 5571,
  [SMALL_STATE(253)] = 5578,
  [SMALL_STATE(254)] = 5585,
  [SMALL_STATE(255)] = 5592,
  [SMALL_STATE(256)] = 5599,
  [SMALL_STATE(257)] = 5606,
  [SMALL_STATE(258)] = 5613,
  [SMALL_STATE(259)] = 5620,
  [SMALL_STATE(260)] = 5627,
  [SMALL_STATE(261)] = 5634,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(242),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(123),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(226),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(259),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(86),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(85),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(227),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(245),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding_ref, 1, 0, 0),
  [45] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_binding_ref, 1, 0, 0), SHIFT(11),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [52] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [54] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binding_ref, 1, 0, 0),
  [56] = {.entry = {.count = 1, .reusable = false}}, SHIFT(52),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anonymous_field, 1, 0, 0),
  [62] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [64] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [66] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [68] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [70] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [72] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym_binding_ref, 1, 0, 0), SHIFT(55),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(214),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [83] = {.entry = {.count = 1, .reusable = false}}, SHIFT(63),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [91] = {.entry = {.count = 1, .reusable = false}}, SHIFT(212),
  [93] = {.entry = {.count = 1, .reusable = false}}, SHIFT(211),
  [95] = {.entry = {.count = 1, .reusable = false}}, SHIFT(83),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [99] = {.entry = {.count = 1, .reusable = false}}, SHIFT(98),
  [101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [105] = {.entry = {.count = 1, .reusable = false}}, SHIFT(260),
  [107] = {.entry = {.count = 1, .reusable = false}}, SHIFT(254),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [111] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assoc_list, 3, 0, 0),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assoc_list, 3, 0, 0),
  [117] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assoc_list, 4, 0, 0),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assoc_list, 4, 0, 0),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_base_type, 1, 0, 0),
  [123] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_base_type, 1, 0, 0),
  [125] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [129] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(242),
  [132] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(123),
  [135] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(226),
  [138] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(167),
  [141] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(218),
  [144] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(259),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_field_access, 3, 0, 0),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_field_access, 3, 0, 0),
  [151] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_or, 3, 0, 0),
  [155] = {.entry = {.count = 1, .reusable = true}}, SHIFT(191),
  [157] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [159] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_and, 3, 0, 0),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_not, 2, 0, 0),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_not, 2, 0, 0),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_comparison, 3, 0, 0),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_comparison, 3, 0, 0),
  [171] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_var, 1, 0, 0),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_var, 1, 0, 0),
  [175] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_call, 4, 0, 0),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_call, 4, 0, 0),
  [179] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_literal, 1, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_literal, 1, 0, 0),
  [183] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_call, 6, 0, 0),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_call, 6, 0, 0),
  [187] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_call, 8, 0, 0),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_call, 8, 0, 0),
  [191] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_paren, 3, 0, 0),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_paren, 3, 0, 0),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_annotation, 3, 0, 0),
  [197] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_value, 3, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_value, 3, 0, 0),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_value, 3, 0, 0),
  [205] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_value, 3, 0, 0),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal_value, 1, 0, 0),
  [209] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal_value, 1, 0, 0),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_type, 3, 0, 0),
  [213] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_type, 3, 0, 0),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_type, 3, 0, 0),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_type, 3, 0, 0),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_intersection_type, 3, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_intersection_type, 3, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [225] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type, 3, 0, 0),
  [227] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type, 3, 0, 0),
  [229] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_reference_type, 2, 0, 0),
  [231] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_reference_type, 2, 0, 0),
  [233] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_ref, 1, 0, 0),
  [235] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type_ref, 1, 0, 0),
  [237] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_concrete_type, 4, 0, 0),
  [239] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_concrete_type, 4, 0, 0),
  [241] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_type, 4, 0, 0),
  [243] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_type, 4, 0, 0),
  [245] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_refinable_ref, 2, 0, 0),
  [247] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_refinable_ref, 2, 0, 0),
  [249] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_type, 2, 0, 0),
  [251] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_type, 2, 0, 0),
  [253] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_named_type, 1, 0, 0),
  [255] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_named_type, 1, 0, 0),
  [257] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_annotation, 3, 0, 0),
  [259] = {.entry = {.count = 1, .reusable = false}}, SHIFT(45),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(204),
  [263] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [267] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [269] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal_type, 1, 0, 0),
  [271] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal_type, 1, 0, 0),
  [273] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_or, 3, 0, 0),
  [275] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_and, 3, 0, 0),
  [277] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variant_value, 2, 0, 0),
  [279] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variant_value, 2, 0, 0),
  [281] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_value, 2, 0, 0),
  [283] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_value, 2, 0, 0),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_value, 2, 0, 0),
  [287] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_value, 2, 0, 0),
  [289] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [291] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [293] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [295] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [297] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [299] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [301] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [303] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [305] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [307] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [309] = {.entry = {.count = 1, .reusable = true}}, SHIFT(216),
  [311] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [313] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [315] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [317] = {.entry = {.count = 1, .reusable = true}}, SHIFT(257),
  [319] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_decl, 4, 0, 3),
  [321] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type_decl, 4, 0, 3),
  [323] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [325] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_decl, 5, 0, 7),
  [327] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type_decl, 5, 0, 7),
  [329] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_stmt, 2, 0, 1),
  [331] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_stmt, 2, 0, 1),
  [333] = {.entry = {.count = 1, .reusable = false}}, SHIFT(219),
  [335] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instance, 4, 0, 2),
  [337] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instance, 4, 0, 2),
  [339] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instance, 7, 0, 16),
  [341] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instance, 7, 0, 16),
  [343] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instance, 5, 0, 6),
  [345] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instance, 5, 0, 6),
  [347] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instance, 6, 0, 11),
  [349] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instance, 6, 0, 11),
  [351] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import_stmt, 4, 0, 4),
  [353] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_import_stmt, 4, 0, 4),
  [355] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instance, 6, 0, 10),
  [357] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instance, 6, 0, 10),
  [359] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instance, 5, 0, 5),
  [361] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instance, 5, 0, 5),
  [363] = {.entry = {.count = 1, .reusable = true}}, SHIFT(198),
  [365] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [367] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [369] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [371] = {.entry = {.count = 1, .reusable = true}}, SHIFT(248),
  [373] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [375] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [377] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_path_repeat1, 2, 0, 0),
  [379] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_path_repeat1, 2, 0, 0), SHIFT_REPEAT(247),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(252),
  [386] = {.entry = {.count = 1, .reusable = true}}, SHIFT(210),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
  [392] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [394] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [396] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 2, 0, 8),
  [398] = {.entry = {.count = 1, .reusable = true}}, SHIFT(153),
  [400] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 3, 0, 13),
  [402] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 3, 0, 12),
  [404] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_origin_path, 1, 0, 0),
  [406] = {.entry = {.count = 1, .reusable = true}}, SHIFT(247),
  [408] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 4, 0, 17),
  [410] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_origin_path, 2, 0, 0),
  [412] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_annotation_args_repeat1, 2, 0, 0),
  [414] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_annotation_args_repeat1, 2, 0, 0), SHIFT_REPEAT(222),
  [417] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field, 2, 0, 9),
  [419] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_path, 1, 0, 0),
  [421] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field, 3, 0, 14),
  [423] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field, 3, 0, 15),
  [425] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field, 4, 0, 18),
  [427] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_path, 2, 0, 0),
  [429] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_field_repeat1, 2, 0, 0),
  [431] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_field_repeat1, 2, 0, 0), SHIFT_REPEAT(126),
  [434] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [436] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [438] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_field_repeat1, 2, 0, 0), SHIFT_REPEAT(123),
  [441] = {.entry = {.count = 1, .reusable = true}}, SHIFT(195),
  [443] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [445] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [447] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [449] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [451] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field_list, 2, 0, 0),
  [453] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [455] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [457] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [459] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_list_elements_repeat1, 2, 0, 0),
  [461] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_elements_repeat1, 2, 0, 0), SHIFT_REPEAT(24),
  [464] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_elements, 2, 0, 0),
  [466] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [468] = {.entry = {.count = 1, .reusable = false}}, SHIFT(132),
  [470] = {.entry = {.count = 1, .reusable = false}}, SHIFT(234),
  [472] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field_list, 1, 0, 0),
  [474] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_value_field_list_repeat1, 2, 0, 0),
  [476] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_value_field_list_repeat1, 2, 0, 0), SHIFT_REPEAT(138),
  [479] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_annotation_args, 2, 0, 0),
  [481] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_args_repeat1, 2, 0, 0),
  [483] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_args_repeat1, 2, 0, 0), SHIFT_REPEAT(205),
  [486] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_anonymous_fields_repeat1, 2, 0, 0),
  [488] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_anonymous_fields_repeat1, 2, 0, 0), SHIFT_REPEAT(190),
  [491] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_field_list_repeat1, 2, 0, 0),
  [493] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_field_list_repeat1, 2, 0, 0), SHIFT_REPEAT(127),
  [496] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [498] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [500] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_annotation_args, 1, 0, 0),
  [502] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [504] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_args, 2, 0, 0),
  [506] = {.entry = {.count = 1, .reusable = true}}, SHIFT(205),
  [508] = {.entry = {.count = 1, .reusable = true}}, SHIFT(184),
  [510] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_origin_paths, 1, 0, 0),
  [512] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [514] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_origin_paths, 2, 0, 0),
  [516] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_origin_paths_repeat1, 2, 0, 0), SHIFT_REPEAT(184),
  [519] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_origin_paths_repeat1, 2, 0, 0),
  [521] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anonymous_fields, 2, 0, 0),
  [523] = {.entry = {.count = 1, .reusable = true}}, SHIFT(190),
  [525] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [527] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_list, 2, 0, 0),
  [529] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [531] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anonymous_fields, 1, 0, 0),
  [533] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_list, 1, 0, 0),
  [535] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_elements, 1, 0, 0),
  [537] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_args, 1, 0, 0),
  [539] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_refinement, 3, 0, 0),
  [541] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 5, 0, 17),
  [543] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_annotation, 5, 0, 0),
  [545] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_origin, 5, 0, 0),
  [547] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 4, 0, 12),
  [549] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_annotation, 2, 0, 0),
  [551] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [553] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [555] = {.entry = {.count = 1, .reusable = false}}, SHIFT(215),
  [557] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 3, 0, 8),
  [559] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cardinality, 1, 0, 0),
  [561] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [563] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_origin, 2, 0, 0),
  [565] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [567] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cardinality, 2, 0, 0),
  [569] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 4, 0, 13),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [573] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_origin, 1, 0, 0),
  [575] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_element, 1, 0, 0),
  [577] = {.entry = {.count = 1, .reusable = false}}, SHIFT(92),
  [579] = {.entry = {.count = 1, .reusable = false}}, SHIFT(255),
  [581] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anonymous_field, 2, 0, 0),
  [583] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [585] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [587] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [589] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_doc_annotation, 2, 0, 0),
  [591] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [593] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [595] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [597] = {.entry = {.count = 1, .reusable = true}}, SHIFT(221),
  [599] = {.entry = {.count = 1, .reusable = true}}, SHIFT(213),
  [601] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [603] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [605] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [607] = {.entry = {.count = 1, .reusable = true}}, SHIFT(162),
  [609] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(246),
  [613] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [615] = {.entry = {.count = 1, .reusable = true}}, SHIFT(253),
  [617] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [619] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cardinality, 3, 0, 0),
  [621] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [623] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [625] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [627] = {.entry = {.count = 1, .reusable = true}}, SHIFT(201),
  [629] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(164),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [637] = {.entry = {.count = 1, .reusable = true}}, SHIFT(192),
  [639] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(249),
  [645] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [647] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [649] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [651] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [653] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [655] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [657] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [659] = {.entry = {.count = 1, .reusable = true}}, SHIFT(236),
  [661] = {.entry = {.count = 1, .reusable = true}}, SHIFT(250),
  [663] = {.entry = {.count = 1, .reusable = true}}, SHIFT(206),
  [665] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [667] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [669] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [671] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [673] = {.entry = {.count = 1, .reusable = true}}, SHIFT(256),
  [675] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [677] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [679] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [681] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_ilk_kli(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
