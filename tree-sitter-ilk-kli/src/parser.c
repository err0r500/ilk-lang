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
#define STATE_COUNT 275
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 122
#define ALIAS_COUNT 0
#define TOKEN_COUNT 59
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 4
#define MAX_ALIAS_SEQUENCE_LENGTH 8
#define PRODUCTION_ID_COUNT 15

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
  anon_sym_LBRACE = 23,
  anon_sym_DOT_DOT_DOT = 24,
  anon_sym_RBRACE = 25,
  anon_sym_COMMA = 26,
  aux_sym_anonymous_fields_token1 = 27,
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
  anon_sym_ATdoc = 56,
  anon_sym_EQ = 57,
  anon_sym_compute = 58,
  sym_source_file = 59,
  sym__definition = 60,
  sym_boolean = 61,
  sym_base_type = 62,
  sym_type_expr = 63,
  sym__primary_type = 64,
  sym_named_type = 65,
  sym_literal_type = 66,
  sym_concrete_type = 67,
  sym_list_type = 68,
  sym_cardinality = 69,
  sym_reference_type = 70,
  sym_struct_type = 71,
  sym_anonymous_fields = 72,
  sym_anonymous_field = 73,
  sym_ilk_field_list = 74,
  sym_ilk_field = 75,
  sym_union_type = 76,
  sym_intersection_type = 77,
  sym_annotation = 78,
  sym_annotation_args = 79,
  sym_source_args = 80,
  sym_source_path = 81,
  sym_constraint_expr = 82,
  sym_constraint_or = 83,
  sym_constraint_and = 84,
  sym_constraint_not = 85,
  sym_constraint_comparison = 86,
  sym__constraint_primary = 87,
  sym_constraint_paren = 88,
  sym_constraint_call = 89,
  sym_constraint_field_access = 90,
  sym_constraint_var = 91,
  sym_constraint_literal = 92,
  sym_block = 93,
  sym_binding = 94,
  sym_assoc_list = 95,
  sym_value = 96,
  sym_type_ref = 97,
  sym_literal_value = 98,
  sym_binding_ref = 99,
  sym_struct_value = 100,
  sym_value_field_list = 101,
  sym_value_field = 102,
  sym_field_origin = 103,
  sym_origin_path = 104,
  sym_origin_paths = 105,
  sym_list_value = 106,
  sym_list_elements = 107,
  sym_list_element = 108,
  sym_refinement = 109,
  sym_variant_value = 110,
  aux_sym_source_file_repeat1 = 111,
  aux_sym_anonymous_fields_repeat1 = 112,
  aux_sym_ilk_field_list_repeat1 = 113,
  aux_sym_ilk_field_repeat1 = 114,
  aux_sym_annotation_args_repeat1 = 115,
  aux_sym_source_args_repeat1 = 116,
  aux_sym_source_path_repeat1 = 117,
  aux_sym_assoc_list_repeat1 = 118,
  aux_sym_value_field_list_repeat1 = 119,
  aux_sym_origin_paths_repeat1 = 120,
  aux_sym_list_elements_repeat1 = 121,
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
  [anon_sym_LBRACE] = "{",
  [anon_sym_DOT_DOT_DOT] = "...",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COMMA] = ",",
  [aux_sym_anonymous_fields_token1] = "anonymous_fields_token1",
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
  [anon_sym_ATdoc] = "@doc",
  [anon_sym_EQ] = "=",
  [anon_sym_compute] = "compute",
  [sym_source_file] = "source_file",
  [sym__definition] = "_definition",
  [sym_boolean] = "boolean",
  [sym_base_type] = "base_type",
  [sym_type_expr] = "type_expr",
  [sym__primary_type] = "_primary_type",
  [sym_named_type] = "named_type",
  [sym_literal_type] = "literal_type",
  [sym_concrete_type] = "concrete_type",
  [sym_list_type] = "list_type",
  [sym_cardinality] = "cardinality",
  [sym_reference_type] = "reference_type",
  [sym_struct_type] = "struct_type",
  [sym_anonymous_fields] = "anonymous_fields",
  [sym_anonymous_field] = "anonymous_field",
  [sym_ilk_field_list] = "ilk_field_list",
  [sym_ilk_field] = "ilk_field",
  [sym_union_type] = "union_type",
  [sym_intersection_type] = "intersection_type",
  [sym_annotation] = "annotation",
  [sym_annotation_args] = "annotation_args",
  [sym_source_args] = "source_args",
  [sym_source_path] = "source_path",
  [sym_constraint_expr] = "constraint_expr",
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
  [sym_block] = "block",
  [sym_binding] = "binding",
  [sym_assoc_list] = "assoc_list",
  [sym_value] = "value",
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
  [aux_sym_ilk_field_list_repeat1] = "ilk_field_list_repeat1",
  [aux_sym_ilk_field_repeat1] = "ilk_field_repeat1",
  [aux_sym_annotation_args_repeat1] = "annotation_args_repeat1",
  [aux_sym_source_args_repeat1] = "source_args_repeat1",
  [aux_sym_source_path_repeat1] = "source_path_repeat1",
  [aux_sym_assoc_list_repeat1] = "assoc_list_repeat1",
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
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_DOT_DOT_DOT] = anon_sym_DOT_DOT_DOT,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [aux_sym_anonymous_fields_token1] = aux_sym_anonymous_fields_token1,
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
  [anon_sym_ATdoc] = anon_sym_ATdoc,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_compute] = anon_sym_compute,
  [sym_source_file] = sym_source_file,
  [sym__definition] = sym__definition,
  [sym_boolean] = sym_boolean,
  [sym_base_type] = sym_base_type,
  [sym_type_expr] = sym_type_expr,
  [sym__primary_type] = sym__primary_type,
  [sym_named_type] = sym_named_type,
  [sym_literal_type] = sym_literal_type,
  [sym_concrete_type] = sym_concrete_type,
  [sym_list_type] = sym_list_type,
  [sym_cardinality] = sym_cardinality,
  [sym_reference_type] = sym_reference_type,
  [sym_struct_type] = sym_struct_type,
  [sym_anonymous_fields] = sym_anonymous_fields,
  [sym_anonymous_field] = sym_anonymous_field,
  [sym_ilk_field_list] = sym_ilk_field_list,
  [sym_ilk_field] = sym_ilk_field,
  [sym_union_type] = sym_union_type,
  [sym_intersection_type] = sym_intersection_type,
  [sym_annotation] = sym_annotation,
  [sym_annotation_args] = sym_annotation_args,
  [sym_source_args] = sym_source_args,
  [sym_source_path] = sym_source_path,
  [sym_constraint_expr] = sym_constraint_expr,
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
  [sym_block] = sym_block,
  [sym_binding] = sym_binding,
  [sym_assoc_list] = sym_assoc_list,
  [sym_value] = sym_value,
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
  [aux_sym_ilk_field_list_repeat1] = aux_sym_ilk_field_list_repeat1,
  [aux_sym_ilk_field_repeat1] = aux_sym_ilk_field_repeat1,
  [aux_sym_annotation_args_repeat1] = aux_sym_annotation_args_repeat1,
  [aux_sym_source_args_repeat1] = aux_sym_source_args_repeat1,
  [aux_sym_source_path_repeat1] = aux_sym_source_path_repeat1,
  [aux_sym_assoc_list_repeat1] = aux_sym_assoc_list_repeat1,
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
  [aux_sym_anonymous_fields_token1] = {
    .visible = false,
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
  [anon_sym_ATdoc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
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
  [sym_type_expr] = {
    .visible = true,
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
  [sym_ilk_field_list] = {
    .visible = true,
    .named = true,
  },
  [sym_ilk_field] = {
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
  [sym_constraint_expr] = {
    .visible = true,
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
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_binding] = {
    .visible = true,
    .named = true,
  },
  [sym_assoc_list] = {
    .visible = true,
    .named = true,
  },
  [sym_value] = {
    .visible = true,
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
  [aux_sym_ilk_field_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_ilk_field_repeat1] = {
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
  [aux_sym_assoc_list_repeat1] = {
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
  field_body = 1,
  field_name = 2,
  field_type = 3,
  field_value = 4,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_body] = "body",
  [field_name] = "name",
  [field_type] = "type",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 2},
  [3] = {.index = 4, .length = 2},
  [4] = {.index = 6, .length = 3},
  [5] = {.index = 9, .length = 2},
  [6] = {.index = 11, .length = 2},
  [7] = {.index = 13, .length = 3},
  [8] = {.index = 16, .length = 2},
  [9] = {.index = 18, .length = 2},
  [10] = {.index = 20, .length = 3},
  [11] = {.index = 23, .length = 2},
  [12] = {.index = 25, .length = 3},
  [13] = {.index = 28, .length = 2},
  [14] = {.index = 30, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_body, 1},
    {field_name, 0},
  [2] =
    {field_body, 2},
    {field_name, 1},
  [4] =
    {field_name, 0},
    {field_type, 1},
  [6] =
    {field_name, 0},
    {field_type, 2},
    {field_value, 3},
  [9] =
    {field_name, 0},
    {field_type, 2},
  [11] =
    {field_name, 1},
    {field_type, 2},
  [13] =
    {field_name, 0},
    {field_type, 2},
    {field_value, 4},
  [16] =
    {field_name, 1},
    {field_type, 3},
  [18] =
    {field_name, 0},
    {field_value, 1},
  [20] =
    {field_name, 2},
    {field_type, 4},
    {field_value, 5},
  [23] =
    {field_name, 0},
    {field_value, 2},
  [25] =
    {field_name, 2},
    {field_type, 4},
    {field_value, 6},
  [28] =
    {field_name, 2},
    {field_value, 3},
  [30] =
    {field_name, 2},
    {field_value, 4},
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
  [12] = 7,
  [13] = 8,
  [14] = 11,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 15,
  [19] = 17,
  [20] = 17,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 22,
  [26] = 23,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 39,
  [41] = 41,
  [42] = 34,
  [43] = 36,
  [44] = 38,
  [45] = 45,
  [46] = 46,
  [47] = 46,
  [48] = 48,
  [49] = 37,
  [50] = 31,
  [51] = 33,
  [52] = 45,
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
  [63] = 55,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 56,
  [69] = 57,
  [70] = 66,
  [71] = 58,
  [72] = 59,
  [73] = 60,
  [74] = 61,
  [75] = 62,
  [76] = 67,
  [77] = 64,
  [78] = 65,
  [79] = 79,
  [80] = 80,
  [81] = 79,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 83,
  [88] = 85,
  [89] = 84,
  [90] = 90,
  [91] = 82,
  [92] = 90,
  [93] = 86,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 79,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 79,
  [105] = 96,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
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
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 100,
  [135] = 102,
  [136] = 108,
  [137] = 99,
  [138] = 98,
  [139] = 139,
  [140] = 106,
  [141] = 107,
  [142] = 109,
  [143] = 110,
  [144] = 144,
  [145] = 101,
  [146] = 103,
  [147] = 128,
  [148] = 148,
  [149] = 149,
  [150] = 133,
  [151] = 151,
  [152] = 152,
  [153] = 153,
  [154] = 154,
  [155] = 155,
  [156] = 113,
  [157] = 115,
  [158] = 158,
  [159] = 159,
  [160] = 114,
  [161] = 161,
  [162] = 162,
  [163] = 163,
  [164] = 164,
  [165] = 165,
  [166] = 166,
  [167] = 167,
  [168] = 168,
  [169] = 118,
  [170] = 170,
  [171] = 171,
  [172] = 120,
  [173] = 173,
  [174] = 174,
  [175] = 121,
  [176] = 119,
  [177] = 177,
  [178] = 132,
  [179] = 179,
  [180] = 126,
  [181] = 181,
  [182] = 182,
  [183] = 182,
  [184] = 184,
  [185] = 185,
  [186] = 123,
  [187] = 187,
  [188] = 188,
  [189] = 189,
  [190] = 190,
  [191] = 191,
  [192] = 192,
  [193] = 193,
  [194] = 194,
  [195] = 195,
  [196] = 196,
  [197] = 197,
  [198] = 198,
  [199] = 199,
  [200] = 200,
  [201] = 201,
  [202] = 202,
  [203] = 130,
  [204] = 204,
  [205] = 205,
  [206] = 206,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 210,
  [211] = 211,
  [212] = 212,
  [213] = 213,
  [214] = 210,
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
  [227] = 224,
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
  [238] = 238,
  [239] = 239,
  [240] = 240,
  [241] = 241,
  [242] = 242,
  [243] = 243,
  [244] = 244,
  [245] = 245,
  [246] = 246,
  [247] = 247,
  [248] = 236,
  [249] = 249,
  [250] = 250,
  [251] = 251,
  [252] = 252,
  [253] = 246,
  [254] = 254,
  [255] = 255,
  [256] = 234,
  [257] = 247,
  [258] = 258,
  [259] = 259,
  [260] = 260,
  [261] = 261,
  [262] = 262,
  [263] = 263,
  [264] = 264,
  [265] = 255,
  [266] = 243,
  [267] = 262,
  [268] = 254,
  [269] = 258,
  [270] = 251,
  [271] = 271,
  [272] = 272,
  [273] = 261,
  [274] = 249,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(132);
      ADVANCE_MAP(
        '!', 284,
        '"', 7,
        '&', 258,
        '(', 291,
        ')', 292,
        '*', 231,
        ',', 262,
        '-', 128,
        '.', 280,
        '/', 16,
        '<', 251,
        '=', 308,
        '>', 253,
        '?', 267,
        '@', 271,
        'B', 84,
        'C', 81,
        'D', 22,
        'F', 61,
        'I', 70,
        'M', 83,
        'S', 105,
        'T', 50,
        'U', 118,
        '[', 254,
        ']', 255,
        '_', 265,
        'a', 99,
        'c', 76,
        'd', 77,
        'e', 124,
        'f', 21,
        'i', 67,
        'k', 36,
        'm', 24,
        'o', 120,
        's', 80,
        't', 45,
        'u', 72,
        '{', 259,
        '|', 269,
        '}', 261,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      END_STATE();
    case 1:
      ADVANCE_MAP(
        '\n', 263,
        '\r', 1,
        '"', 7,
        '&', 258,
        '*', 231,
        ',', 262,
        '-', 128,
        '.', 279,
        '/', 16,
        '=', 307,
        'B', 183,
        'C', 181,
        'D', 137,
        'F', 167,
        'I', 178,
        'M', 185,
        'S', 210,
        'T', 159,
        'U', 216,
        '[', 254,
        ']', 255,
        'f', 136,
        't', 193,
        '{', 259,
        '|', 268,
        '}', 261,
      );
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 2:
      ADVANCE_MAP(
        '\n', 264,
        '\r', 2,
        '"', 7,
        '&', 258,
        '*', 231,
        ',', 262,
        '-', 128,
        '/', 16,
        '=', 307,
        'B', 183,
        'D', 137,
        'F', 167,
        'I', 178,
        'M', 185,
        'S', 210,
        'T', 159,
        'U', 216,
        '[', 254,
        ']', 255,
        'f', 136,
        't', 193,
        '{', 259,
        '}', 261,
      );
      if (('\t' <= lookahead && lookahead <= '\f') ||
          lookahead == ' ') SKIP(2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 3:
      ADVANCE_MAP(
        '!', 283,
        '"', 7,
        '(', 291,
        '-', 128,
        '/', 16,
        'c', 186,
        'e', 221,
        'f', 135,
        'k', 155,
        't', 156,
        'u', 179,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 4:
      ADVANCE_MAP(
        '!', 17,
        '&', 8,
        ')', 292,
        ',', 262,
        '.', 279,
        '/', 16,
        '<', 251,
        '=', 18,
        '>', 253,
        'i', 67,
        '|', 127,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(4);
      END_STATE();
    case 5:
      ADVANCE_MAP(
        '!', 17,
        '&', 8,
        '.', 279,
        '/', 16,
        '<', 251,
        '=', 18,
        '>', 253,
        '@', 270,
        'i', 176,
        '|', 127,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(5);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 6:
      ADVANCE_MAP(
        '"', 7,
        '&', 258,
        '*', 231,
        '-', 128,
        '.', 10,
        '/', 16,
        '=', 307,
        '?', 267,
        'B', 183,
        'C', 181,
        'D', 137,
        'F', 167,
        'I', 178,
        'M', 185,
        'S', 210,
        'T', 159,
        'U', 216,
        '[', 254,
        ']', 255,
        'f', 136,
        't', 193,
        '{', 259,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(6);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 7:
      if (lookahead == '"') ADVANCE(225);
      if (lookahead == '\\') ADVANCE(129);
      if (lookahead != 0) ADVANCE(7);
      END_STATE();
    case 8:
      if (lookahead == '&') ADVANCE(282);
      END_STATE();
    case 9:
      if (lookahead == '.') ADVANCE(260);
      END_STATE();
    case 10:
      if (lookahead == '.') ADVANCE(256);
      END_STATE();
    case 11:
      if (lookahead == '.') ADVANCE(12);
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '@') ADVANCE(270);
      if (lookahead == '_') ADVANCE(266);
      if (lookahead == '}') ADVANCE(261);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(11);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 12:
      if (lookahead == '.') ADVANCE(9);
      END_STATE();
    case 13:
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == '@') ADVANCE(270);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(13);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 14:
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == 'a') ADVANCE(204);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(14);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 15:
      if (lookahead == '/') ADVANCE(16);
      if (lookahead == 'c') ADVANCE(187);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(15);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 16:
      if (lookahead == '/') ADVANCE(133);
      END_STATE();
    case 17:
      if (lookahead == '=') ADVANCE(286);
      END_STATE();
    case 18:
      if (lookahead == '=') ADVANCE(285);
      END_STATE();
    case 19:
      if (lookahead == '>') ADVANCE(299);
      END_STATE();
    case 20:
      if (lookahead == 'V') ADVANCE(27);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(59);
      if (lookahead == 'o') ADVANCE(90);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(111);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(60);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(54);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(108);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(56);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(93);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(66);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(117);
      END_STATE();
    case 30:
      if (lookahead == 'c') ADVANCE(278);
      END_STATE();
    case 31:
      if (lookahead == 'c') ADVANCE(306);
      END_STATE();
    case 32:
      if (lookahead == 'c') ADVANCE(274);
      END_STATE();
    case 33:
      if (lookahead == 'c') ADVANCE(92);
      END_STATE();
    case 34:
      if (lookahead == 'c') ADVANCE(40);
      END_STATE();
    case 35:
      if (lookahead == 'd') ADVANCE(232);
      END_STATE();
    case 36:
      if (lookahead == 'e') ADVANCE(126);
      END_STATE();
    case 37:
      if (lookahead == 'e') ADVANCE(242);
      END_STATE();
    case 38:
      if (lookahead == 'e') ADVANCE(227);
      END_STATE();
    case 39:
      if (lookahead == 'e') ADVANCE(229);
      END_STATE();
    case 40:
      if (lookahead == 'e') ADVANCE(276);
      END_STATE();
    case 41:
      if (lookahead == 'e') ADVANCE(297);
      END_STATE();
    case 42:
      if (lookahead == 'e') ADVANCE(309);
      END_STATE();
    case 43:
      if (lookahead == 'e') ADVANCE(248);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(20);
      END_STATE();
    case 45:
      if (lookahead == 'e') ADVANCE(64);
      if (lookahead == 'r') ADVANCE(121);
      END_STATE();
    case 46:
      if (lookahead == 'e') ADVANCE(125);
      END_STATE();
    case 47:
      if (lookahead == 'e') ADVANCE(102);
      END_STATE();
    case 48:
      if (lookahead == 'e') ADVANCE(116);
      END_STATE();
    case 49:
      if (lookahead == 'g') ADVANCE(234);
      END_STATE();
    case 50:
      if (lookahead == 'i') ADVANCE(65);
      END_STATE();
    case 51:
      if (lookahead == 'i') ADVANCE(35);
      END_STATE();
    case 52:
      if (lookahead == 'i') ADVANCE(89);
      END_STATE();
    case 53:
      if (lookahead == 'i') ADVANCE(101);
      END_STATE();
    case 54:
      if (lookahead == 'i') ADVANCE(68);
      END_STATE();
    case 55:
      if (lookahead == 'i') ADVANCE(69);
      END_STATE();
    case 56:
      if (lookahead == 'i') ADVANCE(74);
      END_STATE();
    case 57:
      if (lookahead == 'l') ADVANCE(240);
      END_STATE();
    case 58:
      if (lookahead == 'l') ADVANCE(293);
      END_STATE();
    case 59:
      if (lookahead == 'l') ADVANCE(100);
      END_STATE();
    case 60:
      if (lookahead == 'l') ADVANCE(58);
      END_STATE();
    case 61:
      if (lookahead == 'l') ADVANCE(85);
      END_STATE();
    case 62:
      if (lookahead == 'l') ADVANCE(29);
      END_STATE();
    case 63:
      if (lookahead == 'm') ADVANCE(88);
      if (lookahead == 'n') ADVANCE(104);
      if (lookahead == 'u') ADVANCE(73);
      END_STATE();
    case 64:
      if (lookahead == 'm') ADVANCE(87);
      END_STATE();
    case 65:
      if (lookahead == 'm') ADVANCE(47);
      END_STATE();
    case 66:
      if (lookahead == 'm') ADVANCE(86);
      END_STATE();
    case 67:
      if (lookahead == 'n') ADVANCE(289);
      END_STATE();
    case 68:
      if (lookahead == 'n') ADVANCE(272);
      END_STATE();
    case 69:
      if (lookahead == 'n') ADVANCE(49);
      END_STATE();
    case 70:
      if (lookahead == 'n') ADVANCE(106);
      END_STATE();
    case 71:
      if (lookahead == 'n') ADVANCE(33);
      END_STATE();
    case 72:
      if (lookahead == 'n') ADVANCE(52);
      END_STATE();
    case 73:
      if (lookahead == 'n') ADVANCE(109);
      END_STATE();
    case 74:
      if (lookahead == 'n') ADVANCE(110);
      END_STATE();
    case 75:
      if (lookahead == 'n') ADVANCE(46);
      END_STATE();
    case 76:
      if (lookahead == 'o') ADVANCE(63);
      END_STATE();
    case 77:
      if (lookahead == 'o') ADVANCE(30);
      END_STATE();
    case 78:
      if (lookahead == 'o') ADVANCE(31);
      END_STATE();
    case 79:
      if (lookahead == 'o') ADVANCE(57);
      END_STATE();
    case 80:
      if (lookahead == 'o') ADVANCE(119);
      END_STATE();
    case 81:
      if (lookahead == 'o') ADVANCE(71);
      END_STATE();
    case 82:
      if (lookahead == 'o') ADVANCE(32);
      END_STATE();
    case 83:
      if (lookahead == 'o') ADVANCE(75);
      END_STATE();
    case 84:
      if (lookahead == 'o') ADVANCE(79);
      END_STATE();
    case 85:
      if (lookahead == 'o') ADVANCE(25);
      END_STATE();
    case 86:
      if (lookahead == 'p') ADVANCE(244);
      END_STATE();
    case 87:
      if (lookahead == 'p') ADVANCE(62);
      END_STATE();
    case 88:
      if (lookahead == 'p') ADVANCE(123);
      END_STATE();
    case 89:
      if (lookahead == 'q') ADVANCE(122);
      END_STATE();
    case 90:
      if (lookahead == 'r') ADVANCE(23);
      END_STATE();
    case 91:
      if (lookahead == 'r') ADVANCE(34);
      END_STATE();
    case 92:
      if (lookahead == 'r') ADVANCE(48);
      END_STATE();
    case 93:
      if (lookahead == 'r') ADVANCE(98);
      END_STATE();
    case 94:
      if (lookahead == 'r') ADVANCE(26);
      END_STATE();
    case 95:
      if (lookahead == 'r') ADVANCE(55);
      END_STATE();
    case 96:
      if (lookahead == 's') ADVANCE(304);
      END_STATE();
    case 97:
      if (lookahead == 's') ADVANCE(295);
      END_STATE();
    case 98:
      if (lookahead == 's') ADVANCE(302);
      END_STATE();
    case 99:
      if (lookahead == 's') ADVANCE(103);
      END_STATE();
    case 100:
      if (lookahead == 's') ADVANCE(39);
      END_STATE();
    case 101:
      if (lookahead == 's') ADVANCE(114);
      END_STATE();
    case 102:
      if (lookahead == 's') ADVANCE(112);
      END_STATE();
    case 103:
      if (lookahead == 's') ADVANCE(82);
      END_STATE();
    case 104:
      if (lookahead == 's') ADVANCE(113);
      END_STATE();
    case 105:
      if (lookahead == 't') ADVANCE(95);
      END_STATE();
    case 106:
      if (lookahead == 't') ADVANCE(236);
      END_STATE();
    case 107:
      if (lookahead == 't') ADVANCE(273);
      END_STATE();
    case 108:
      if (lookahead == 't') ADVANCE(238);
      END_STATE();
    case 109:
      if (lookahead == 't') ADVANCE(300);
      END_STATE();
    case 110:
      if (lookahead == 't') ADVANCE(277);
      END_STATE();
    case 111:
      if (lookahead == 't') ADVANCE(37);
      END_STATE();
    case 112:
      if (lookahead == 't') ADVANCE(28);
      END_STATE();
    case 113:
      if (lookahead == 't') ADVANCE(94);
      END_STATE();
    case 114:
      if (lookahead == 't') ADVANCE(97);
      END_STATE();
    case 115:
      if (lookahead == 't') ADVANCE(42);
      END_STATE();
    case 116:
      if (lookahead == 't') ADVANCE(43);
      END_STATE();
    case 117:
      if (lookahead == 't') ADVANCE(44);
      END_STATE();
    case 118:
      if (lookahead == 'u') ADVANCE(51);
      END_STATE();
    case 119:
      if (lookahead == 'u') ADVANCE(91);
      END_STATE();
    case 120:
      if (lookahead == 'u') ADVANCE(107);
      END_STATE();
    case 121:
      if (lookahead == 'u') ADVANCE(38);
      END_STATE();
    case 122:
      if (lookahead == 'u') ADVANCE(41);
      END_STATE();
    case 123:
      if (lookahead == 'u') ADVANCE(115);
      END_STATE();
    case 124:
      if (lookahead == 'x') ADVANCE(53);
      END_STATE();
    case 125:
      if (lookahead == 'y') ADVANCE(246);
      END_STATE();
    case 126:
      if (lookahead == 'y') ADVANCE(96);
      END_STATE();
    case 127:
      if (lookahead == '|') ADVANCE(281);
      END_STATE();
    case 128:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      END_STATE();
    case 129:
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(7);
      END_STATE();
    case 130:
      if (eof) ADVANCE(132);
      ADVANCE_MAP(
        '"', 7,
        '*', 231,
        '-', 128,
        '/', 16,
        '<', 250,
        '?', 267,
        '@', 271,
        'B', 183,
        'D', 137,
        'F', 167,
        'I', 178,
        'M', 185,
        'S', 210,
        'T', 159,
        'U', 216,
        '[', 254,
        ']', 255,
        'f', 136,
        't', 193,
        '{', 259,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(130);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 131:
      if (eof) ADVANCE(132);
      ADVANCE_MAP(
        '&', 258,
        ')', 292,
        ',', 262,
        '.', 279,
        '/', 16,
        '=', 19,
        '>', 252,
        '@', 271,
        '|', 268,
        '}', 261,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(131);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(133);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'V') ADVANCE(141);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(164);
      if (lookahead == 'o') ADVANCE(197);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(164);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(206);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(170);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(168);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 140:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(208);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(195);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(214);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 143:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(275);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(196);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'd') ADVANCE(233);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(243);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(222);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(228);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(230);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(249);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(298);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(134);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(310);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(201);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(223);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(171);
      if (lookahead == 'r') ADVANCE(217);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(213);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'g') ADVANCE(235);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(172);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(145);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 161:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(192);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(175);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(205);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(202);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(241);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(294);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(188);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(166);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(142);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(189);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(191);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(154);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'm') ADVANCE(190);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(144);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(158);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(290);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(147);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(207);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(161);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(209);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(174);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(143);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(184);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(165);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(177);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(219);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(173);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(140);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 189:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(245);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(220);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'p') ADVANCE(169);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'q') ADVANCE(218);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(217);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(162);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(200);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(157);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(139);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 198:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(305);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 199:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(296);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 200:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(303);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 201:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(211);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 202:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(149);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 203:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(182);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 204:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(203);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(212);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 206:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(146);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 207:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(237);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 208:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(239);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 209:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(301);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 210:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(194);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 211:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(138);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 212:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(199);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 213:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(150);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(152);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 215:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(153);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(160);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 217:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(148);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(151);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(180);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 220:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(215);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 221:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'x') ADVANCE(163);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 222:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'y') ADVANCE(247);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 223:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'y') ADVANCE(198);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 224:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 225:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 226:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(226);
      END_STATE();
    case 227:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 228:
      ACCEPT_TOKEN(anon_sym_true);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 229:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 230:
      ACCEPT_TOKEN(anon_sym_false);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 231:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 232:
      ACCEPT_TOKEN(anon_sym_Uuid);
      END_STATE();
    case 233:
      ACCEPT_TOKEN(anon_sym_Uuid);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 234:
      ACCEPT_TOKEN(anon_sym_String);
      END_STATE();
    case 235:
      ACCEPT_TOKEN(anon_sym_String);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 236:
      ACCEPT_TOKEN(anon_sym_Int);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(anon_sym_Int);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(anon_sym_Float);
      END_STATE();
    case 239:
      ACCEPT_TOKEN(anon_sym_Float);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 240:
      ACCEPT_TOKEN(anon_sym_Bool);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(anon_sym_Bool);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 242:
      ACCEPT_TOKEN(anon_sym_Date);
      END_STATE();
    case 243:
      ACCEPT_TOKEN(anon_sym_Date);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 244:
      ACCEPT_TOKEN(anon_sym_Timestamp);
      END_STATE();
    case 245:
      ACCEPT_TOKEN(anon_sym_Timestamp);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 246:
      ACCEPT_TOKEN(anon_sym_Money);
      END_STATE();
    case 247:
      ACCEPT_TOKEN(anon_sym_Money);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 248:
      ACCEPT_TOKEN(anon_sym_Concrete);
      END_STATE();
    case 249:
      ACCEPT_TOKEN(anon_sym_Concrete);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 250:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(287);
      END_STATE();
    case 252:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 253:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(288);
      END_STATE();
    case 254:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 255:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 256:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      END_STATE();
    case 257:
      ACCEPT_TOKEN(anon_sym_DOT_DOT);
      if (lookahead == '.') ADVANCE(260);
      END_STATE();
    case 258:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 259:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 260:
      ACCEPT_TOKEN(anon_sym_DOT_DOT_DOT);
      END_STATE();
    case 261:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 262:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 263:
      ACCEPT_TOKEN(aux_sym_anonymous_fields_token1);
      if (lookahead == '\n') ADVANCE(263);
      if (lookahead == '\r') ADVANCE(1);
      END_STATE();
    case 264:
      ACCEPT_TOKEN(aux_sym_anonymous_fields_token1);
      if (lookahead == '\n') ADVANCE(264);
      if (lookahead == '\r') ADVANCE(2);
      END_STATE();
    case 265:
      ACCEPT_TOKEN(anon_sym__);
      END_STATE();
    case 266:
      ACCEPT_TOKEN(anon_sym__);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 267:
      ACCEPT_TOKEN(anon_sym_QMARK);
      END_STATE();
    case 268:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 269:
      ACCEPT_TOKEN(anon_sym_PIPE);
      if (lookahead == '|') ADVANCE(281);
      END_STATE();
    case 270:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead == 'd') ADVANCE(78);
      END_STATE();
    case 272:
      ACCEPT_TOKEN(anon_sym_main);
      END_STATE();
    case 273:
      ACCEPT_TOKEN(anon_sym_out);
      END_STATE();
    case 274:
      ACCEPT_TOKEN(anon_sym_assoc);
      END_STATE();
    case 275:
      ACCEPT_TOKEN(anon_sym_assoc);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 276:
      ACCEPT_TOKEN(anon_sym_source);
      END_STATE();
    case 277:
      ACCEPT_TOKEN(anon_sym_constraint);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(anon_sym_doc);
      END_STATE();
    case 279:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead == '.') ADVANCE(257);
      END_STATE();
    case 281:
      ACCEPT_TOKEN(anon_sym_PIPE_PIPE);
      END_STATE();
    case 282:
      ACCEPT_TOKEN(anon_sym_AMP_AMP);
      END_STATE();
    case 283:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 284:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(286);
      END_STATE();
    case 285:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 286:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 287:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 288:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 289:
      ACCEPT_TOKEN(anon_sym_in);
      END_STATE();
    case 290:
      ACCEPT_TOKEN(anon_sym_in);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 291:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 292:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 293:
      ACCEPT_TOKEN(anon_sym_forall);
      END_STATE();
    case 294:
      ACCEPT_TOKEN(anon_sym_forall);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 295:
      ACCEPT_TOKEN(anon_sym_exists);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_exists);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(anon_sym_unique);
      END_STATE();
    case 298:
      ACCEPT_TOKEN(anon_sym_unique);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 299:
      ACCEPT_TOKEN(anon_sym_EQ_GT);
      END_STATE();
    case 300:
      ACCEPT_TOKEN(anon_sym_count);
      END_STATE();
    case 301:
      ACCEPT_TOKEN(anon_sym_count);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 302:
      ACCEPT_TOKEN(anon_sym_templateVars);
      END_STATE();
    case 303:
      ACCEPT_TOKEN(anon_sym_templateVars);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 304:
      ACCEPT_TOKEN(anon_sym_keys);
      END_STATE();
    case 305:
      ACCEPT_TOKEN(anon_sym_keys);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    case 306:
      ACCEPT_TOKEN(anon_sym_ATdoc);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 308:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(285);
      if (lookahead == '>') ADVANCE(299);
      END_STATE();
    case 309:
      ACCEPT_TOKEN(anon_sym_compute);
      END_STATE();
    case 310:
      ACCEPT_TOKEN(anon_sym_compute);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(224);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 131},
  [2] = {.lex_state = 6},
  [3] = {.lex_state = 6},
  [4] = {.lex_state = 6},
  [5] = {.lex_state = 6},
  [6] = {.lex_state = 1},
  [7] = {.lex_state = 6},
  [8] = {.lex_state = 6},
  [9] = {.lex_state = 6},
  [10] = {.lex_state = 6},
  [11] = {.lex_state = 6},
  [12] = {.lex_state = 6},
  [13] = {.lex_state = 6},
  [14] = {.lex_state = 6},
  [15] = {.lex_state = 130},
  [16] = {.lex_state = 2},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 130},
  [19] = {.lex_state = 130},
  [20] = {.lex_state = 2},
  [21] = {.lex_state = 130},
  [22] = {.lex_state = 6},
  [23] = {.lex_state = 6},
  [24] = {.lex_state = 130},
  [25] = {.lex_state = 6},
  [26] = {.lex_state = 6},
  [27] = {.lex_state = 130},
  [28] = {.lex_state = 130},
  [29] = {.lex_state = 130},
  [30] = {.lex_state = 130},
  [31] = {.lex_state = 3},
  [32] = {.lex_state = 130},
  [33] = {.lex_state = 3},
  [34] = {.lex_state = 3},
  [35] = {.lex_state = 130},
  [36] = {.lex_state = 3},
  [37] = {.lex_state = 3},
  [38] = {.lex_state = 3},
  [39] = {.lex_state = 3},
  [40] = {.lex_state = 3},
  [41] = {.lex_state = 130},
  [42] = {.lex_state = 3},
  [43] = {.lex_state = 3},
  [44] = {.lex_state = 3},
  [45] = {.lex_state = 3},
  [46] = {.lex_state = 3},
  [47] = {.lex_state = 3},
  [48] = {.lex_state = 3},
  [49] = {.lex_state = 3},
  [50] = {.lex_state = 3},
  [51] = {.lex_state = 3},
  [52] = {.lex_state = 3},
  [53] = {.lex_state = 130},
  [54] = {.lex_state = 130},
  [55] = {.lex_state = 4},
  [56] = {.lex_state = 5},
  [57] = {.lex_state = 5},
  [58] = {.lex_state = 5},
  [59] = {.lex_state = 5},
  [60] = {.lex_state = 5},
  [61] = {.lex_state = 5},
  [62] = {.lex_state = 5},
  [63] = {.lex_state = 5},
  [64] = {.lex_state = 5},
  [65] = {.lex_state = 5},
  [66] = {.lex_state = 4},
  [67] = {.lex_state = 4},
  [68] = {.lex_state = 4},
  [69] = {.lex_state = 4},
  [70] = {.lex_state = 5},
  [71] = {.lex_state = 4},
  [72] = {.lex_state = 4},
  [73] = {.lex_state = 4},
  [74] = {.lex_state = 4},
  [75] = {.lex_state = 4},
  [76] = {.lex_state = 5},
  [77] = {.lex_state = 4},
  [78] = {.lex_state = 4},
  [79] = {.lex_state = 4},
  [80] = {.lex_state = 5},
  [81] = {.lex_state = 5},
  [82] = {.lex_state = 11},
  [83] = {.lex_state = 4},
  [84] = {.lex_state = 4},
  [85] = {.lex_state = 4},
  [86] = {.lex_state = 4},
  [87] = {.lex_state = 4},
  [88] = {.lex_state = 4},
  [89] = {.lex_state = 4},
  [90] = {.lex_state = 4},
  [91] = {.lex_state = 11},
  [92] = {.lex_state = 4},
  [93] = {.lex_state = 4},
  [94] = {.lex_state = 131},
  [95] = {.lex_state = 131},
  [96] = {.lex_state = 1},
  [97] = {.lex_state = 1},
  [98] = {.lex_state = 131},
  [99] = {.lex_state = 131},
  [100] = {.lex_state = 131},
  [101] = {.lex_state = 131},
  [102] = {.lex_state = 131},
  [103] = {.lex_state = 131},
  [104] = {.lex_state = 131},
  [105] = {.lex_state = 131},
  [106] = {.lex_state = 131},
  [107] = {.lex_state = 131},
  [108] = {.lex_state = 131},
  [109] = {.lex_state = 131},
  [110] = {.lex_state = 131},
  [111] = {.lex_state = 1},
  [112] = {.lex_state = 1},
  [113] = {.lex_state = 1},
  [114] = {.lex_state = 1},
  [115] = {.lex_state = 1},
  [116] = {.lex_state = 0},
  [117] = {.lex_state = 1},
  [118] = {.lex_state = 1},
  [119] = {.lex_state = 1},
  [120] = {.lex_state = 1},
  [121] = {.lex_state = 1},
  [122] = {.lex_state = 1},
  [123] = {.lex_state = 1},
  [124] = {.lex_state = 131},
  [125] = {.lex_state = 131},
  [126] = {.lex_state = 1},
  [127] = {.lex_state = 13},
  [128] = {.lex_state = 131},
  [129] = {.lex_state = 1},
  [130] = {.lex_state = 1},
  [131] = {.lex_state = 1},
  [132] = {.lex_state = 1},
  [133] = {.lex_state = 1},
  [134] = {.lex_state = 1},
  [135] = {.lex_state = 1},
  [136] = {.lex_state = 1},
  [137] = {.lex_state = 1},
  [138] = {.lex_state = 1},
  [139] = {.lex_state = 1},
  [140] = {.lex_state = 1},
  [141] = {.lex_state = 1},
  [142] = {.lex_state = 1},
  [143] = {.lex_state = 1},
  [144] = {.lex_state = 1},
  [145] = {.lex_state = 1},
  [146] = {.lex_state = 1},
  [147] = {.lex_state = 131},
  [148] = {.lex_state = 1},
  [149] = {.lex_state = 1},
  [150] = {.lex_state = 131},
  [151] = {.lex_state = 1},
  [152] = {.lex_state = 1},
  [153] = {.lex_state = 1},
  [154] = {.lex_state = 131},
  [155] = {.lex_state = 1},
  [156] = {.lex_state = 131},
  [157] = {.lex_state = 131},
  [158] = {.lex_state = 1},
  [159] = {.lex_state = 1},
  [160] = {.lex_state = 131},
  [161] = {.lex_state = 1},
  [162] = {.lex_state = 13},
  [163] = {.lex_state = 1},
  [164] = {.lex_state = 1},
  [165] = {.lex_state = 1},
  [166] = {.lex_state = 13},
  [167] = {.lex_state = 1},
  [168] = {.lex_state = 131},
  [169] = {.lex_state = 131},
  [170] = {.lex_state = 1},
  [171] = {.lex_state = 1},
  [172] = {.lex_state = 131},
  [173] = {.lex_state = 1},
  [174] = {.lex_state = 13},
  [175] = {.lex_state = 131},
  [176] = {.lex_state = 131},
  [177] = {.lex_state = 1},
  [178] = {.lex_state = 131},
  [179] = {.lex_state = 131},
  [180] = {.lex_state = 131},
  [181] = {.lex_state = 1},
  [182] = {.lex_state = 6},
  [183] = {.lex_state = 6},
  [184] = {.lex_state = 131},
  [185] = {.lex_state = 1},
  [186] = {.lex_state = 131},
  [187] = {.lex_state = 1},
  [188] = {.lex_state = 1},
  [189] = {.lex_state = 1},
  [190] = {.lex_state = 1},
  [191] = {.lex_state = 15},
  [192] = {.lex_state = 1},
  [193] = {.lex_state = 131},
  [194] = {.lex_state = 1},
  [195] = {.lex_state = 1},
  [196] = {.lex_state = 131},
  [197] = {.lex_state = 0},
  [198] = {.lex_state = 1},
  [199] = {.lex_state = 0},
  [200] = {.lex_state = 1},
  [201] = {.lex_state = 0},
  [202] = {.lex_state = 1},
  [203] = {.lex_state = 131},
  [204] = {.lex_state = 0},
  [205] = {.lex_state = 1},
  [206] = {.lex_state = 1},
  [207] = {.lex_state = 1},
  [208] = {.lex_state = 1},
  [209] = {.lex_state = 131},
  [210] = {.lex_state = 131},
  [211] = {.lex_state = 1},
  [212] = {.lex_state = 0},
  [213] = {.lex_state = 1},
  [214] = {.lex_state = 131},
  [215] = {.lex_state = 0},
  [216] = {.lex_state = 1},
  [217] = {.lex_state = 13},
  [218] = {.lex_state = 13},
  [219] = {.lex_state = 0},
  [220] = {.lex_state = 0},
  [221] = {.lex_state = 131},
  [222] = {.lex_state = 131},
  [223] = {.lex_state = 0},
  [224] = {.lex_state = 14},
  [225] = {.lex_state = 131},
  [226] = {.lex_state = 0},
  [227] = {.lex_state = 14},
  [228] = {.lex_state = 0},
  [229] = {.lex_state = 13},
  [230] = {.lex_state = 6},
  [231] = {.lex_state = 0},
  [232] = {.lex_state = 0},
  [233] = {.lex_state = 0},
  [234] = {.lex_state = 0},
  [235] = {.lex_state = 0},
  [236] = {.lex_state = 0},
  [237] = {.lex_state = 0},
  [238] = {.lex_state = 0},
  [239] = {.lex_state = 0},
  [240] = {.lex_state = 131},
  [241] = {.lex_state = 131},
  [242] = {.lex_state = 0},
  [243] = {.lex_state = 0},
  [244] = {.lex_state = 131},
  [245] = {.lex_state = 131},
  [246] = {.lex_state = 131},
  [247] = {.lex_state = 131},
  [248] = {.lex_state = 0},
  [249] = {.lex_state = 0},
  [250] = {.lex_state = 0},
  [251] = {.lex_state = 131},
  [252] = {.lex_state = 131},
  [253] = {.lex_state = 131},
  [254] = {.lex_state = 0},
  [255] = {.lex_state = 0},
  [256] = {.lex_state = 0},
  [257] = {.lex_state = 131},
  [258] = {.lex_state = 131},
  [259] = {.lex_state = 0},
  [260] = {.lex_state = 0},
  [261] = {.lex_state = 0},
  [262] = {.lex_state = 0},
  [263] = {.lex_state = 131},
  [264] = {.lex_state = 0},
  [265] = {.lex_state = 0},
  [266] = {.lex_state = 0},
  [267] = {.lex_state = 0},
  [268] = {.lex_state = 0},
  [269] = {.lex_state = 131},
  [270] = {.lex_state = 131},
  [271] = {.lex_state = 131},
  [272] = {.lex_state = 6},
  [273] = {.lex_state = 0},
  [274] = {.lex_state = 0},
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
    [anon_sym_assoc] = ACTIONS(1),
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
    [anon_sym_ATdoc] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_compute] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(260),
    [sym__definition] = STATE(95),
    [sym_annotation] = STATE(166),
    [sym_block] = STATE(95),
    [sym_binding] = STATE(95),
    [aux_sym_source_file_repeat1] = STATE(95),
    [aux_sym_ilk_field_repeat1] = STATE(166),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [sym_identifier] = ACTIONS(7),
    [anon_sym_AT] = ACTIONS(9),
    [anon_sym_ATdoc] = ACTIONS(11),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_Concrete,
    ACTIONS(25), 1,
      anon_sym_LBRACK,
    ACTIONS(27), 1,
      anon_sym_AMP,
    ACTIONS(29), 1,
      anon_sym_LBRACE,
    ACTIONS(31), 1,
      anon_sym_EQ,
    STATE(102), 1,
      sym_boolean,
    STATE(125), 1,
      sym_type_expr,
    ACTIONS(15), 2,
      sym_string,
      sym_number,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(108), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [61] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(45), 1,
      anon_sym_LBRACK,
    ACTIONS(47), 1,
      anon_sym_AMP,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    ACTIONS(51), 1,
      anon_sym_QMARK,
    STATE(135), 1,
      sym_boolean,
    STATE(149), 1,
      sym_type_expr,
    ACTIONS(35), 2,
      sym_string,
      sym_number,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(136), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [122] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(45), 1,
      anon_sym_LBRACK,
    ACTIONS(47), 1,
      anon_sym_AMP,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    ACTIONS(53), 1,
      anon_sym_QMARK,
    STATE(135), 1,
      sym_boolean,
    STATE(144), 1,
      sym_type_expr,
    ACTIONS(35), 2,
      sym_string,
      sym_number,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(136), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [183] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(45), 1,
      anon_sym_LBRACK,
    ACTIONS(47), 1,
      anon_sym_AMP,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    STATE(129), 1,
      sym_type_expr,
    STATE(135), 1,
      sym_boolean,
    ACTIONS(35), 2,
      sym_string,
      sym_number,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(136), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [241] = 13,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(59), 1,
      anon_sym_LBRACK,
    ACTIONS(61), 1,
      anon_sym_AMP,
    ACTIONS(63), 1,
      anon_sym_LBRACE,
    ACTIONS(67), 1,
      aux_sym_anonymous_fields_token1,
    STATE(135), 1,
      sym_boolean,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(57), 2,
      sym_string,
      sym_number,
    ACTIONS(65), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
    STATE(188), 8,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
    ACTIONS(41), 9,
      anon_sym_STAR,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [299] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_Concrete,
    ACTIONS(25), 1,
      anon_sym_LBRACK,
    ACTIONS(27), 1,
      anon_sym_AMP,
    ACTIONS(29), 1,
      anon_sym_LBRACE,
    STATE(102), 1,
      sym_boolean,
    STATE(110), 1,
      sym_type_expr,
    ACTIONS(15), 2,
      sym_string,
      sym_number,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(108), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [357] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_Concrete,
    ACTIONS(25), 1,
      anon_sym_LBRACK,
    ACTIONS(27), 1,
      anon_sym_AMP,
    ACTIONS(29), 1,
      anon_sym_LBRACE,
    STATE(102), 1,
      sym_boolean,
    STATE(109), 1,
      sym_type_expr,
    ACTIONS(15), 2,
      sym_string,
      sym_number,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(108), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [415] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_Concrete,
    ACTIONS(25), 1,
      anon_sym_LBRACK,
    ACTIONS(27), 1,
      anon_sym_AMP,
    ACTIONS(29), 1,
      anon_sym_LBRACE,
    STATE(102), 1,
      sym_boolean,
    STATE(124), 1,
      sym_type_expr,
    ACTIONS(15), 2,
      sym_string,
      sym_number,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(108), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [473] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(45), 1,
      anon_sym_LBRACK,
    ACTIONS(47), 1,
      anon_sym_AMP,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    STATE(135), 1,
      sym_boolean,
    STATE(139), 1,
      sym_type_expr,
    ACTIONS(35), 2,
      sym_string,
      sym_number,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(136), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [531] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_Concrete,
    ACTIONS(25), 1,
      anon_sym_LBRACK,
    ACTIONS(27), 1,
      anon_sym_AMP,
    ACTIONS(29), 1,
      anon_sym_LBRACE,
    STATE(102), 1,
      sym_boolean,
    STATE(214), 1,
      sym_type_expr,
    ACTIONS(15), 2,
      sym_string,
      sym_number,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(108), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [589] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(45), 1,
      anon_sym_LBRACK,
    ACTIONS(47), 1,
      anon_sym_AMP,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    STATE(135), 1,
      sym_boolean,
    STATE(143), 1,
      sym_type_expr,
    ACTIONS(35), 2,
      sym_string,
      sym_number,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(136), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [647] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(45), 1,
      anon_sym_LBRACK,
    ACTIONS(47), 1,
      anon_sym_AMP,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    STATE(135), 1,
      sym_boolean,
    STATE(142), 1,
      sym_type_expr,
    ACTIONS(35), 2,
      sym_string,
      sym_number,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(136), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [705] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_Concrete,
    ACTIONS(25), 1,
      anon_sym_LBRACK,
    ACTIONS(27), 1,
      anon_sym_AMP,
    ACTIONS(29), 1,
      anon_sym_LBRACE,
    STATE(102), 1,
      sym_boolean,
    STATE(210), 1,
      sym_type_expr,
    ACTIONS(15), 2,
      sym_string,
      sym_number,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(108), 10,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
      sym_union_type,
      sym_intersection_type,
  [763] = 15,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(69), 1,
      sym_identifier,
    ACTIONS(73), 1,
      anon_sym_LBRACK,
    ACTIONS(75), 1,
      anon_sym_RBRACK,
    ACTIONS(77), 1,
      anon_sym_LBRACE,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    STATE(173), 1,
      sym_list_element,
    STATE(265), 1,
      sym_list_elements,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(71), 2,
      sym_string,
      sym_number,
    STATE(200), 2,
      sym_value,
      sym_refinement,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [824] = 14,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(79), 1,
      sym_identifier,
    ACTIONS(83), 1,
      anon_sym_LBRACK,
    ACTIONS(87), 1,
      anon_sym_AMP,
    ACTIONS(89), 1,
      anon_sym_LBRACE,
    ACTIONS(91), 1,
      aux_sym_anonymous_fields_token1,
    STATE(118), 1,
      sym_value,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(81), 2,
      sym_string,
      sym_number,
    ACTIONS(85), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 9,
      anon_sym_STAR,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [883] = 14,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(83), 1,
      anon_sym_LBRACK,
    ACTIONS(89), 1,
      anon_sym_LBRACE,
    ACTIONS(91), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(93), 1,
      sym_identifier,
    ACTIONS(95), 1,
      anon_sym_STAR,
    STATE(118), 1,
      sym_value,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(81), 2,
      sym_string,
      sym_number,
    ACTIONS(85), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [942] = 15,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(69), 1,
      sym_identifier,
    ACTIONS(73), 1,
      anon_sym_LBRACK,
    ACTIONS(77), 1,
      anon_sym_LBRACE,
    ACTIONS(98), 1,
      anon_sym_RBRACK,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    STATE(173), 1,
      sym_list_element,
    STATE(255), 1,
      sym_list_elements,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(71), 2,
      sym_string,
      sym_number,
    STATE(200), 2,
      sym_value,
      sym_refinement,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1003] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(85), 1,
      anon_sym_AT,
    ACTIONS(100), 1,
      sym_identifier,
    ACTIONS(105), 1,
      anon_sym_LBRACK,
    ACTIONS(107), 1,
      anon_sym_LBRACE,
    STATE(169), 1,
      sym_value,
    STATE(180), 1,
      sym_base_type,
    STATE(186), 1,
      sym_boolean,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(91), 2,
      ts_builtin_sym_end,
      anon_sym_ATdoc,
    ACTIONS(103), 2,
      sym_string,
      sym_number,
    STATE(157), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1061] = 13,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(79), 1,
      sym_identifier,
    ACTIONS(83), 1,
      anon_sym_LBRACK,
    ACTIONS(89), 1,
      anon_sym_LBRACE,
    ACTIONS(91), 1,
      aux_sym_anonymous_fields_token1,
    STATE(118), 1,
      sym_value,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(81), 2,
      sym_string,
      sym_number,
    ACTIONS(85), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 9,
      anon_sym_STAR,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1117] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(105), 1,
      anon_sym_LBRACK,
    ACTIONS(107), 1,
      anon_sym_LBRACE,
    ACTIONS(109), 1,
      sym_identifier,
    ACTIONS(111), 1,
      anon_sym_LT,
    STATE(41), 1,
      sym_assoc_list,
    STATE(168), 1,
      sym_value,
    STATE(180), 1,
      sym_base_type,
    STATE(186), 1,
      sym_boolean,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(103), 2,
      sym_string,
      sym_number,
    STATE(157), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1174] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_Concrete,
    ACTIONS(25), 1,
      anon_sym_LBRACK,
    ACTIONS(27), 1,
      anon_sym_AMP,
    ACTIONS(29), 1,
      anon_sym_LBRACE,
    STATE(102), 1,
      sym_boolean,
    ACTIONS(15), 2,
      sym_string,
      sym_number,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(103), 8,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
  [1227] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(45), 1,
      anon_sym_LBRACK,
    ACTIONS(47), 1,
      anon_sym_AMP,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    STATE(135), 1,
      sym_boolean,
    ACTIONS(35), 2,
      sym_string,
      sym_number,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(140), 8,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
  [1280] = 14,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(105), 1,
      anon_sym_LBRACK,
    ACTIONS(107), 1,
      anon_sym_LBRACE,
    ACTIONS(109), 1,
      sym_identifier,
    ACTIONS(111), 1,
      anon_sym_LT,
    STATE(32), 1,
      sym_assoc_list,
    STATE(180), 1,
      sym_base_type,
    STATE(184), 1,
      sym_value,
    STATE(186), 1,
      sym_boolean,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(103), 2,
      sym_string,
      sym_number,
    STATE(157), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1337] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 1,
      sym_identifier,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(43), 1,
      anon_sym_Concrete,
    ACTIONS(45), 1,
      anon_sym_LBRACK,
    ACTIONS(47), 1,
      anon_sym_AMP,
    ACTIONS(49), 1,
      anon_sym_LBRACE,
    STATE(135), 1,
      sym_boolean,
    ACTIONS(35), 2,
      sym_string,
      sym_number,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(146), 8,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
  [1390] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(13), 1,
      sym_identifier,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(23), 1,
      anon_sym_Concrete,
    ACTIONS(25), 1,
      anon_sym_LBRACK,
    ACTIONS(27), 1,
      anon_sym_AMP,
    ACTIONS(29), 1,
      anon_sym_LBRACE,
    STATE(102), 1,
      sym_boolean,
    ACTIONS(15), 2,
      sym_string,
      sym_number,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
    STATE(106), 8,
      sym_base_type,
      sym__primary_type,
      sym_named_type,
      sym_literal_type,
      sym_concrete_type,
      sym_list_type,
      sym_reference_type,
      sym_struct_type,
  [1443] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(69), 1,
      sym_identifier,
    ACTIONS(73), 1,
      anon_sym_LBRACK,
    ACTIONS(77), 1,
      anon_sym_LBRACE,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    STATE(207), 1,
      sym_list_element,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(71), 2,
      sym_string,
      sym_number,
    STATE(200), 2,
      sym_value,
      sym_refinement,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1498] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(73), 1,
      anon_sym_LBRACK,
    ACTIONS(77), 1,
      anon_sym_LBRACE,
    ACTIONS(93), 1,
      sym_identifier,
    ACTIONS(113), 1,
      anon_sym_QMARK,
    STATE(111), 1,
      sym_value,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(71), 2,
      sym_string,
      sym_number,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1552] = 13,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(73), 1,
      anon_sym_LBRACK,
    ACTIONS(77), 1,
      anon_sym_LBRACE,
    ACTIONS(93), 1,
      sym_identifier,
    ACTIONS(115), 1,
      anon_sym_QMARK,
    STATE(117), 1,
      sym_value,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(71), 2,
      sym_string,
      sym_number,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1606] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(73), 1,
      anon_sym_LBRACK,
    ACTIONS(77), 1,
      anon_sym_LBRACE,
    ACTIONS(93), 1,
      sym_identifier,
    STATE(112), 1,
      sym_value,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(71), 2,
      sym_string,
      sym_number,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1657] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(83), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [1706] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(105), 1,
      anon_sym_LBRACK,
    ACTIONS(107), 1,
      anon_sym_LBRACE,
    ACTIONS(109), 1,
      sym_identifier,
    STATE(154), 1,
      sym_value,
    STATE(180), 1,
      sym_base_type,
    STATE(186), 1,
      sym_boolean,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(103), 2,
      sym_string,
      sym_number,
    STATE(157), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1757] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(86), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [1806] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(131), 1,
      sym_identifier,
    ACTIONS(137), 1,
      anon_sym_BANG,
    ACTIONS(139), 1,
      anon_sym_LPAREN,
    STATE(59), 1,
      sym_constraint_expr,
    STATE(76), 1,
      sym_boolean,
    ACTIONS(133), 2,
      sym_string,
      sym_number,
    ACTIONS(135), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(141), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(143), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(56), 10,
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
  [1855] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(39), 1,
      anon_sym_STAR,
    ACTIONS(73), 1,
      anon_sym_LBRACK,
    ACTIONS(77), 1,
      anon_sym_LBRACE,
    ACTIONS(93), 1,
      sym_identifier,
    STATE(122), 1,
      sym_value,
    STATE(123), 1,
      sym_boolean,
    STATE(126), 1,
      sym_base_type,
    ACTIONS(37), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(71), 2,
      sym_string,
      sym_number,
    STATE(115), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(41), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [1906] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(131), 1,
      sym_identifier,
    ACTIONS(137), 1,
      anon_sym_BANG,
    ACTIONS(139), 1,
      anon_sym_LPAREN,
    STATE(61), 1,
      sym_constraint_expr,
    STATE(76), 1,
      sym_boolean,
    ACTIONS(133), 2,
      sym_string,
      sym_number,
    ACTIONS(135), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(141), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(143), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(56), 10,
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
  [1955] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(85), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2004] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(131), 1,
      sym_identifier,
    ACTIONS(137), 1,
      anon_sym_BANG,
    ACTIONS(139), 1,
      anon_sym_LPAREN,
    STATE(62), 1,
      sym_constraint_expr,
    STATE(76), 1,
      sym_boolean,
    ACTIONS(133), 2,
      sym_string,
      sym_number,
    ACTIONS(135), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(141), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(143), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(56), 10,
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
  [2053] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(69), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2102] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(131), 1,
      sym_identifier,
    ACTIONS(137), 1,
      anon_sym_BANG,
    ACTIONS(139), 1,
      anon_sym_LPAREN,
    STATE(57), 1,
      sym_constraint_expr,
    STATE(76), 1,
      sym_boolean,
    ACTIONS(133), 2,
      sym_string,
      sym_number,
    ACTIONS(135), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(141), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(143), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(56), 10,
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
  [2151] = 12,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(19), 1,
      anon_sym_STAR,
    ACTIONS(105), 1,
      anon_sym_LBRACK,
    ACTIONS(107), 1,
      anon_sym_LBRACE,
    ACTIONS(109), 1,
      sym_identifier,
    STATE(179), 1,
      sym_value,
    STATE(180), 1,
      sym_base_type,
    STATE(186), 1,
      sym_boolean,
    ACTIONS(17), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(103), 2,
      sym_string,
      sym_number,
    STATE(157), 6,
      sym_type_ref,
      sym_literal_value,
      sym_binding_ref,
      sym_struct_value,
      sym_list_value,
      sym_variant_value,
    ACTIONS(21), 8,
      anon_sym_Uuid,
      anon_sym_String,
      anon_sym_Int,
      anon_sym_Float,
      anon_sym_Bool,
      anon_sym_Date,
      anon_sym_Timestamp,
      anon_sym_Money,
  [2202] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(72), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2251] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(74), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2300] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(75), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2349] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(89), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2398] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(92), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2447] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(90), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2496] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(131), 1,
      sym_identifier,
    ACTIONS(137), 1,
      anon_sym_BANG,
    ACTIONS(139), 1,
      anon_sym_LPAREN,
    STATE(76), 1,
      sym_boolean,
    STATE(80), 1,
      sym_constraint_expr,
    ACTIONS(133), 2,
      sym_string,
      sym_number,
    ACTIONS(135), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(141), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(143), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(56), 10,
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
  [2545] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(88), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2594] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(87), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2643] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(93), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2692] = 11,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(117), 1,
      sym_identifier,
    ACTIONS(123), 1,
      anon_sym_BANG,
    ACTIONS(125), 1,
      anon_sym_LPAREN,
    STATE(67), 1,
      sym_boolean,
    STATE(84), 1,
      sym_constraint_expr,
    ACTIONS(119), 2,
      sym_string,
      sym_number,
    ACTIONS(121), 2,
      anon_sym_true,
      anon_sym_false,
    ACTIONS(127), 3,
      anon_sym_forall,
      anon_sym_exists,
      anon_sym_unique,
    ACTIONS(129), 3,
      anon_sym_count,
      anon_sym_templateVars,
      anon_sym_keys,
    STATE(68), 10,
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
  [2741] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(147), 5,
      sym_string,
      sym_number,
      anon_sym_STAR,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
    ACTIONS(145), 11,
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
  [2765] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(151), 5,
      sym_string,
      sym_number,
      anon_sym_STAR,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
    ACTIONS(149), 11,
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
  [2789] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(153), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(155), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [2809] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(157), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(159), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2829] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 1,
      anon_sym_DOT,
    ACTIONS(161), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(163), 7,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2851] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(167), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(169), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2871] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 1,
      anon_sym_DOT,
    ACTIONS(171), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(173), 7,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2893] = 3,
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
  [2913] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 1,
      anon_sym_DOT,
    ACTIONS(179), 1,
      sym_identifier,
    ACTIONS(185), 1,
      anon_sym_AMP_AMP,
    ACTIONS(183), 2,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
    ACTIONS(181), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(187), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2941] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 1,
      anon_sym_DOT,
    ACTIONS(189), 1,
      sym_identifier,
    ACTIONS(181), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(191), 3,
      anon_sym_AT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
    ACTIONS(187), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2967] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(153), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(155), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [2987] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(193), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(195), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3007] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(197), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(199), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3027] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(203), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3047] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(207), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3067] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(157), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(159), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3087] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(161), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(163), 9,
      anon_sym_COMMA,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3109] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(201), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(203), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3129] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(167), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(169), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3149] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(171), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(173), 9,
      anon_sym_COMMA,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3171] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(175), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(177), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3191] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(183), 3,
      anon_sym_COMMA,
      anon_sym_PIPE_PIPE,
      anon_sym_RPAREN,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3217] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(191), 4,
      anon_sym_COMMA,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_RPAREN,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3241] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(205), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(207), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3261] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(193), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(195), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3281] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(197), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(199), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3301] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(219), 10,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
      anon_sym_RPAREN,
  [3321] = 8,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(165), 1,
      anon_sym_DOT,
    ACTIONS(185), 1,
      anon_sym_AMP_AMP,
    ACTIONS(221), 1,
      sym_identifier,
    ACTIONS(223), 1,
      anon_sym_AT,
    ACTIONS(225), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(181), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(187), 4,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3351] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 4,
      sym_identifier,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_in,
    ACTIONS(219), 8,
      anon_sym_AT,
      anon_sym_DOT,
      anon_sym_PIPE_PIPE,
      anon_sym_AMP_AMP,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
  [3371] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(227), 1,
      sym_identifier,
    ACTIONS(229), 1,
      anon_sym_DOT_DOT_DOT,
    ACTIONS(231), 1,
      anon_sym_RBRACE,
    ACTIONS(233), 1,
      anon_sym__,
    ACTIONS(235), 1,
      anon_sym_AT,
    STATE(159), 1,
      sym_anonymous_field,
    STATE(161), 1,
      sym_ilk_field,
    STATE(162), 2,
      sym_annotation,
      aux_sym_ilk_field_repeat1,
    STATE(274), 2,
      sym_anonymous_fields,
      sym_ilk_field_list,
  [3404] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(239), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3431] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(241), 1,
      anon_sym_COMMA,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3458] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(243), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3485] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(245), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3512] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(247), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3539] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(249), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3566] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(251), 1,
      anon_sym_COMMA,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3593] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(253), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3620] = 10,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(227), 1,
      sym_identifier,
    ACTIONS(233), 1,
      anon_sym__,
    ACTIONS(235), 1,
      anon_sym_AT,
    ACTIONS(255), 1,
      anon_sym_DOT_DOT_DOT,
    ACTIONS(257), 1,
      anon_sym_RBRACE,
    STATE(159), 1,
      sym_anonymous_field,
    STATE(161), 1,
      sym_ilk_field,
    STATE(162), 2,
      sym_annotation,
      aux_sym_ilk_field_repeat1,
    STATE(249), 2,
      sym_anonymous_fields,
      sym_ilk_field_list,
  [3653] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(259), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3680] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(209), 1,
      anon_sym_DOT,
    ACTIONS(213), 1,
      anon_sym_AMP_AMP,
    ACTIONS(237), 1,
      anon_sym_PIPE_PIPE,
    ACTIONS(261), 1,
      anon_sym_RPAREN,
    ACTIONS(211), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(215), 5,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_in,
  [3707] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(263), 1,
      ts_builtin_sym_end,
    ACTIONS(265), 1,
      sym_identifier,
    ACTIONS(268), 1,
      anon_sym_AT,
    ACTIONS(271), 1,
      anon_sym_ATdoc,
    STATE(166), 2,
      sym_annotation,
      aux_sym_ilk_field_repeat1,
    STATE(94), 4,
      sym__definition,
      sym_block,
      sym_binding,
      aux_sym_source_file_repeat1,
  [3733] = 7,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(9), 1,
      anon_sym_AT,
    ACTIONS(11), 1,
      anon_sym_ATdoc,
    ACTIONS(274), 1,
      ts_builtin_sym_end,
    STATE(166), 2,
      sym_annotation,
      aux_sym_ilk_field_repeat1,
    STATE(94), 4,
      sym__definition,
      sym_block,
      sym_binding,
      aux_sym_source_file_repeat1,
  [3759] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(278), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(276), 7,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_EQ,
  [3775] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(219), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(217), 7,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_EQ,
  [3791] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(282), 1,
      anon_sym_AT,
    ACTIONS(280), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3806] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(286), 1,
      anon_sym_AT,
    ACTIONS(284), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3821] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(290), 1,
      anon_sym_AT,
    ACTIONS(288), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3836] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(294), 1,
      anon_sym_AT,
    ACTIONS(292), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3851] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(298), 1,
      anon_sym_AT,
    ACTIONS(296), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3866] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(302), 1,
      anon_sym_AT,
    ACTIONS(300), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3881] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(217), 1,
      anon_sym_AT,
    ACTIONS(219), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3896] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(276), 1,
      anon_sym_AT,
    ACTIONS(278), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3911] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(306), 1,
      anon_sym_AT,
    ACTIONS(304), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3926] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(310), 1,
      anon_sym_AT,
    ACTIONS(308), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3941] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(314), 1,
      anon_sym_AT,
    ACTIONS(312), 6,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3956] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 1,
      anon_sym_AMP,
    ACTIONS(320), 1,
      anon_sym_AT,
    ACTIONS(316), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3973] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 1,
      anon_sym_AMP,
    ACTIONS(324), 1,
      anon_sym_AT,
    ACTIONS(322), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_GT,
      anon_sym_PIPE,
      anon_sym_ATdoc,
  [3990] = 6,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(326), 1,
      anon_sym_STAR,
    ACTIONS(330), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(332), 1,
      anon_sym_EQ,
    STATE(205), 1,
      sym_field_origin,
    ACTIONS(328), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4010] = 6,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(326), 1,
      anon_sym_STAR,
    ACTIONS(332), 1,
      anon_sym_EQ,
    ACTIONS(336), 1,
      aux_sym_anonymous_fields_token1,
    STATE(211), 1,
      sym_field_origin,
    ACTIONS(334), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4030] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(340), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(338), 5,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4044] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(344), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(342), 5,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4058] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(348), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(346), 5,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4072] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(352), 1,
      anon_sym_assoc,
    ACTIONS(354), 1,
      anon_sym_source,
    ACTIONS(356), 1,
      anon_sym_constraint,
    ACTIONS(358), 1,
      anon_sym_doc,
    ACTIONS(350), 2,
      anon_sym_main,
      anon_sym_out,
  [4092] = 6,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(326), 1,
      anon_sym_STAR,
    ACTIONS(332), 1,
      anon_sym_EQ,
    ACTIONS(362), 1,
      aux_sym_anonymous_fields_token1,
    STATE(194), 1,
      sym_field_origin,
    ACTIONS(360), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4112] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(366), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(364), 5,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4126] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(370), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(372), 1,
      anon_sym_DOT,
    STATE(119), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(368), 3,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4144] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(377), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(375), 5,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4158] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(381), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(379), 5,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4172] = 6,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(326), 1,
      anon_sym_STAR,
    ACTIONS(332), 1,
      anon_sym_EQ,
    ACTIONS(385), 1,
      aux_sym_anonymous_fields_token1,
    STATE(198), 1,
      sym_field_origin,
    ACTIONS(383), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4192] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(389), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(387), 5,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4206] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 1,
      anon_sym_AMP,
    ACTIONS(393), 1,
      anon_sym_PIPE,
    ACTIONS(395), 1,
      anon_sym_AT,
    ACTIONS(391), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4224] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 1,
      anon_sym_AMP,
    ACTIONS(393), 1,
      anon_sym_PIPE,
    ACTIONS(399), 1,
      anon_sym_AT,
    ACTIONS(397), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4242] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(403), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(401), 5,
      anon_sym_STAR,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_EQ,
  [4256] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 1,
      anon_sym_AT,
    ACTIONS(405), 1,
      sym_identifier,
    STATE(192), 1,
      sym_ilk_field,
    STATE(162), 2,
      sym_annotation,
      aux_sym_ilk_field_repeat1,
  [4273] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(407), 1,
      sym_identifier,
    ACTIONS(409), 1,
      anon_sym_RBRACE,
    ACTIONS(411), 1,
      anon_sym_ATdoc,
    STATE(177), 1,
      sym_value_field,
    STATE(234), 1,
      sym_value_field_list,
  [4292] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(413), 1,
      anon_sym_AMP,
    ACTIONS(417), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(419), 1,
      anon_sym_PIPE,
    ACTIONS(415), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4309] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(370), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(368), 4,
      anon_sym_RBRACK,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_DOT,
  [4322] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(423), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(425), 1,
      anon_sym_DOT,
    STATE(148), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(421), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [4339] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(425), 1,
      anon_sym_DOT,
    ACTIONS(429), 1,
      aux_sym_anonymous_fields_token1,
    STATE(133), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(427), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4356] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(425), 1,
      anon_sym_DOT,
    ACTIONS(433), 1,
      aux_sym_anonymous_fields_token1,
    STATE(119), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(431), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4373] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(288), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(290), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4386] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(296), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(298), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4399] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(312), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(314), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4412] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(284), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(286), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4425] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(280), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(282), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4438] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(413), 1,
      anon_sym_AMP,
    ACTIONS(419), 1,
      anon_sym_PIPE,
    ACTIONS(437), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(435), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4455] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(304), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(306), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4468] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(308), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(310), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4481] = 4,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(316), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(413), 1,
      anon_sym_AMP,
    ACTIONS(320), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4496] = 4,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(322), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(413), 1,
      anon_sym_AMP,
    ACTIONS(324), 3,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4511] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(413), 1,
      anon_sym_AMP,
    ACTIONS(419), 1,
      anon_sym_PIPE,
    ACTIONS(441), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(439), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4528] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(292), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(294), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4541] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(300), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(302), 4,
      anon_sym_AMP,
      anon_sym_RBRACE,
      anon_sym_COMMA,
      anon_sym_PIPE,
  [4554] = 6,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(407), 1,
      sym_identifier,
    ACTIONS(411), 1,
      anon_sym_ATdoc,
    ACTIONS(443), 1,
      anon_sym_RBRACE,
    STATE(177), 1,
      sym_value_field,
    STATE(256), 1,
      sym_value_field_list,
  [4573] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(425), 1,
      anon_sym_DOT,
    ACTIONS(447), 1,
      aux_sym_anonymous_fields_token1,
    STATE(119), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(445), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [4590] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(413), 1,
      anon_sym_AMP,
    ACTIONS(419), 1,
      anon_sym_PIPE,
    ACTIONS(451), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(449), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [4607] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(453), 1,
      anon_sym_DOT,
    STATE(176), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(433), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [4621] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(455), 1,
      anon_sym_RBRACK,
    ACTIONS(457), 1,
      anon_sym_COMMA,
    ACTIONS(460), 1,
      aux_sym_anonymous_fields_token1,
    STATE(151), 1,
      aux_sym_list_elements_repeat1,
  [4637] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(463), 1,
      anon_sym_RBRACE,
    ACTIONS(465), 1,
      anon_sym_COMMA,
    ACTIONS(468), 1,
      aux_sym_anonymous_fields_token1,
    STATE(152), 1,
      aux_sym_value_field_list_repeat1,
  [4653] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(471), 1,
      anon_sym_RBRACK,
    ACTIONS(473), 1,
      anon_sym_COMMA,
    ACTIONS(475), 1,
      aux_sym_anonymous_fields_token1,
    STATE(185), 1,
      aux_sym_source_args_repeat1,
  [4669] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(479), 1,
      anon_sym_AT,
    ACTIONS(477), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4681] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(481), 1,
      anon_sym_RBRACE,
    ACTIONS(483), 1,
      anon_sym_COMMA,
    ACTIONS(485), 1,
      aux_sym_anonymous_fields_token1,
    STATE(187), 1,
      aux_sym_anonymous_fields_repeat1,
  [4697] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(338), 1,
      anon_sym_AT,
    ACTIONS(340), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4709] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(346), 1,
      anon_sym_AT,
    ACTIONS(348), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4721] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(487), 1,
      anon_sym_RBRACK,
    ACTIONS(489), 1,
      anon_sym_COMMA,
    ACTIONS(491), 1,
      aux_sym_anonymous_fields_token1,
    STATE(151), 1,
      aux_sym_list_elements_repeat1,
  [4737] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(483), 1,
      anon_sym_COMMA,
    ACTIONS(485), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(493), 1,
      anon_sym_RBRACE,
    STATE(155), 1,
      aux_sym_anonymous_fields_repeat1,
  [4753] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(342), 1,
      anon_sym_AT,
    ACTIONS(344), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4765] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(495), 1,
      anon_sym_RBRACE,
    ACTIONS(497), 1,
      anon_sym_COMMA,
    ACTIONS(499), 1,
      aux_sym_anonymous_fields_token1,
    STATE(163), 1,
      aux_sym_ilk_field_list_repeat1,
  [4781] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 1,
      anon_sym_AT,
    ACTIONS(501), 1,
      sym_identifier,
    STATE(174), 2,
      sym_annotation,
      aux_sym_ilk_field_repeat1,
  [4795] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(497), 1,
      anon_sym_COMMA,
    ACTIONS(499), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(503), 1,
      anon_sym_RBRACE,
    STATE(167), 1,
      aux_sym_ilk_field_list_repeat1,
  [4811] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(505), 1,
      anon_sym_RBRACE,
    ACTIONS(507), 1,
      anon_sym_COMMA,
    ACTIONS(509), 1,
      aux_sym_anonymous_fields_token1,
    STATE(152), 1,
      aux_sym_value_field_list_repeat1,
  [4827] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(511), 1,
      anon_sym_RBRACK,
    ACTIONS(513), 1,
      anon_sym_COMMA,
    ACTIONS(516), 1,
      aux_sym_anonymous_fields_token1,
    STATE(165), 1,
      aux_sym_annotation_args_repeat1,
  [4843] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(235), 1,
      anon_sym_AT,
    ACTIONS(519), 1,
      sym_identifier,
    STATE(174), 2,
      sym_annotation,
      aux_sym_ilk_field_repeat1,
  [4857] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(521), 1,
      anon_sym_RBRACE,
    ACTIONS(523), 1,
      anon_sym_COMMA,
    ACTIONS(526), 1,
      aux_sym_anonymous_fields_token1,
    STATE(167), 1,
      aux_sym_ilk_field_list_repeat1,
  [4873] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(531), 1,
      anon_sym_AT,
    ACTIONS(529), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4885] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(364), 1,
      anon_sym_AT,
    ACTIONS(366), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4897] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(533), 1,
      anon_sym_RBRACK,
    ACTIONS(535), 1,
      anon_sym_COMMA,
    ACTIONS(538), 1,
      aux_sym_anonymous_fields_token1,
    STATE(170), 1,
      aux_sym_source_args_repeat1,
  [4913] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(541), 1,
      anon_sym_RBRACK,
    ACTIONS(543), 1,
      anon_sym_COMMA,
    ACTIONS(545), 1,
      aux_sym_anonymous_fields_token1,
    STATE(181), 1,
      aux_sym_annotation_args_repeat1,
  [4929] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(375), 1,
      anon_sym_AT,
    ACTIONS(377), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4941] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(489), 1,
      anon_sym_COMMA,
    ACTIONS(491), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(547), 1,
      anon_sym_RBRACK,
    STATE(158), 1,
      aux_sym_list_elements_repeat1,
  [4957] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(549), 1,
      sym_identifier,
    ACTIONS(551), 1,
      anon_sym_AT,
    STATE(174), 2,
      sym_annotation,
      aux_sym_ilk_field_repeat1,
  [4971] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(379), 1,
      anon_sym_AT,
    ACTIONS(381), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [4983] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(554), 1,
      anon_sym_DOT,
    STATE(176), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(370), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [4997] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(507), 1,
      anon_sym_COMMA,
    ACTIONS(509), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(557), 1,
      anon_sym_RBRACE,
    STATE(164), 1,
      aux_sym_value_field_list_repeat1,
  [5013] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(453), 1,
      anon_sym_DOT,
    STATE(150), 1,
      aux_sym_source_path_repeat1,
    ACTIONS(429), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [5027] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(561), 1,
      anon_sym_AT,
    ACTIONS(559), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [5039] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(401), 1,
      anon_sym_AT,
    ACTIONS(403), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [5051] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(543), 1,
      anon_sym_COMMA,
    ACTIONS(545), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(563), 1,
      anon_sym_RBRACK,
    STATE(165), 1,
      aux_sym_annotation_args_repeat1,
  [5067] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(565), 1,
      sym_number,
    ACTIONS(567), 1,
      anon_sym_RBRACK,
    ACTIONS(569), 1,
      anon_sym_DOT_DOT,
    STATE(248), 1,
      sym_cardinality,
  [5083] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(565), 1,
      sym_number,
    ACTIONS(569), 1,
      anon_sym_DOT_DOT,
    ACTIONS(571), 1,
      anon_sym_RBRACK,
    STATE(236), 1,
      sym_cardinality,
  [5099] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(575), 1,
      anon_sym_AT,
    ACTIONS(573), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [5111] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(473), 1,
      anon_sym_COMMA,
    ACTIONS(475), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(577), 1,
      anon_sym_RBRACK,
    STATE(170), 1,
      aux_sym_source_args_repeat1,
  [5127] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(387), 1,
      anon_sym_AT,
    ACTIONS(389), 3,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_ATdoc,
  [5139] = 5,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(579), 1,
      anon_sym_RBRACE,
    ACTIONS(581), 1,
      anon_sym_COMMA,
    ACTIONS(584), 1,
      aux_sym_anonymous_fields_token1,
    STATE(187), 1,
      aux_sym_anonymous_fields_repeat1,
  [5155] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(589), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(587), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5166] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(591), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(511), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5177] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(593), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(579), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5188] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(595), 1,
      sym_identifier,
    ACTIONS(597), 1,
      anon_sym_compute,
    STATE(213), 1,
      sym_origin_path,
  [5201] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(599), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(521), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5212] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(601), 1,
      sym_identifier,
    STATE(197), 1,
      sym_origin_path,
    STATE(264), 1,
      sym_origin_paths,
  [5225] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(605), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(603), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5236] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(607), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(533), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5247] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(609), 1,
      sym_identifier,
    STATE(153), 1,
      sym_source_path,
    STATE(238), 1,
      sym_source_args,
  [5260] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(611), 1,
      anon_sym_COMMA,
    ACTIONS(613), 1,
      anon_sym_RPAREN,
    STATE(201), 1,
      aux_sym_origin_paths_repeat1,
  [5273] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(617), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(615), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5284] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(619), 1,
      anon_sym_GT,
    ACTIONS(621), 1,
      anon_sym_COMMA,
    STATE(212), 1,
      aux_sym_assoc_list_repeat1,
  [5297] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(625), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(623), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5308] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(611), 1,
      anon_sym_COMMA,
    ACTIONS(627), 1,
      anon_sym_RPAREN,
    STATE(204), 1,
      aux_sym_origin_paths_repeat1,
  [5321] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(631), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(629), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5332] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(370), 3,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_RPAREN,
  [5341] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(633), 1,
      anon_sym_COMMA,
    ACTIONS(636), 1,
      anon_sym_RPAREN,
    STATE(204), 1,
      aux_sym_origin_paths_repeat1,
  [5354] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(640), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(638), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5365] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(642), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(463), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5376] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(644), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(455), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5387] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(648), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(646), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5398] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(407), 1,
      sym_identifier,
    ACTIONS(411), 1,
      anon_sym_ATdoc,
    STATE(206), 1,
      sym_value_field,
  [5411] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 1,
      anon_sym_AMP,
    ACTIONS(393), 1,
      anon_sym_PIPE,
    ACTIONS(650), 1,
      anon_sym_GT,
  [5424] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(654), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(652), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5435] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(621), 1,
      anon_sym_COMMA,
    ACTIONS(656), 1,
      anon_sym_GT,
    STATE(215), 1,
      aux_sym_assoc_list_repeat1,
  [5448] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(660), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(658), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [5459] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(318), 1,
      anon_sym_AMP,
    ACTIONS(393), 1,
      anon_sym_PIPE,
    ACTIONS(662), 1,
      anon_sym_GT,
  [5472] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(664), 1,
      anon_sym_GT,
    ACTIONS(666), 1,
      anon_sym_COMMA,
    STATE(215), 1,
      aux_sym_assoc_list_repeat1,
  [5485] = 3,
    ACTIONS(55), 1,
      sym_comment,
    ACTIONS(671), 1,
      aux_sym_anonymous_fields_token1,
    ACTIONS(669), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [5496] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(673), 2,
      sym_identifier,
      anon_sym_AT,
  [5504] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(675), 2,
      sym_identifier,
      anon_sym_AT,
  [5512] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(636), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [5520] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(677), 1,
      sym_number,
    ACTIONS(679), 1,
      anon_sym_RBRACK,
  [5530] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(681), 1,
      sym_identifier,
    STATE(238), 1,
      sym_annotation_args,
  [5540] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(601), 1,
      sym_identifier,
    STATE(219), 1,
      sym_origin_path,
  [5550] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(664), 2,
      anon_sym_GT,
      anon_sym_COMMA,
  [5558] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(683), 1,
      sym_identifier,
    ACTIONS(685), 1,
      anon_sym_assoc,
  [5568] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(609), 1,
      sym_identifier,
    STATE(195), 1,
      sym_source_path,
  [5578] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(77), 1,
      anon_sym_LBRACE,
    STATE(216), 1,
      sym_struct_value,
  [5588] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(687), 1,
      sym_identifier,
    ACTIONS(689), 1,
      anon_sym_assoc,
  [5598] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(691), 1,
      anon_sym__,
    STATE(190), 1,
      sym_anonymous_field,
  [5608] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(223), 2,
      sym_identifier,
      anon_sym_AT,
  [5616] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(693), 1,
      anon_sym_RBRACK,
    ACTIONS(695), 1,
      anon_sym_DOT_DOT,
  [5626] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(697), 1,
      anon_sym_RBRACK,
  [5633] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(699), 1,
      sym_string,
  [5640] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(701), 1,
      sym_string,
  [5647] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(703), 1,
      anon_sym_RBRACE,
  [5654] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(705), 1,
      sym_number,
  [5661] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(707), 1,
      anon_sym_RBRACK,
  [5668] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(709), 1,
      anon_sym_LPAREN,
  [5675] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(711), 1,
      anon_sym_RBRACK,
  [5682] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(679), 1,
      anon_sym_RBRACK,
  [5689] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(713), 1,
      sym_identifier,
  [5696] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(715), 1,
      sym_identifier,
  [5703] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(717), 1,
      anon_sym_LBRACK,
  [5710] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(719), 1,
      anon_sym_LT,
  [5717] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(721), 1,
      sym_identifier,
  [5724] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(723), 1,
      sym_identifier,
  [5731] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(725), 1,
      sym_identifier,
  [5738] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(727), 1,
      sym_identifier,
  [5745] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(729), 1,
      anon_sym_RBRACK,
  [5752] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(731), 1,
      anon_sym_RBRACE,
  [5759] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(733), 1,
      anon_sym_LBRACK,
  [5766] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(735), 1,
      sym_identifier,
  [5773] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(737), 1,
      sym_identifier,
  [5780] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(739), 1,
      sym_identifier,
  [5787] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(741), 1,
      anon_sym_LPAREN,
  [5794] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(743), 1,
      anon_sym_RBRACK,
  [5801] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(745), 1,
      anon_sym_RBRACE,
  [5808] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(747), 1,
      sym_identifier,
  [5815] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(749), 1,
      anon_sym_EQ_GT,
  [5822] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(751), 1,
      sym_string,
  [5829] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(753), 1,
      ts_builtin_sym_end,
  [5836] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(755), 1,
      anon_sym_LPAREN,
  [5843] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(757), 1,
      anon_sym_LPAREN,
  [5850] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(759), 1,
      sym_identifier,
  [5857] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(761), 1,
      anon_sym_RPAREN,
  [5864] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(763), 1,
      anon_sym_RBRACK,
  [5871] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(765), 1,
      anon_sym_LT,
  [5878] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(767), 1,
      anon_sym_LPAREN,
  [5885] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(769), 1,
      anon_sym_LPAREN,
  [5892] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(771), 1,
      anon_sym_EQ_GT,
  [5899] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(773), 1,
      sym_identifier,
  [5906] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(775), 1,
      sym_identifier,
  [5913] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(777), 1,
      anon_sym_EQ,
  [5920] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(779), 1,
      anon_sym_LPAREN,
  [5927] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(781), 1,
      anon_sym_RBRACE,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 61,
  [SMALL_STATE(4)] = 122,
  [SMALL_STATE(5)] = 183,
  [SMALL_STATE(6)] = 241,
  [SMALL_STATE(7)] = 299,
  [SMALL_STATE(8)] = 357,
  [SMALL_STATE(9)] = 415,
  [SMALL_STATE(10)] = 473,
  [SMALL_STATE(11)] = 531,
  [SMALL_STATE(12)] = 589,
  [SMALL_STATE(13)] = 647,
  [SMALL_STATE(14)] = 705,
  [SMALL_STATE(15)] = 763,
  [SMALL_STATE(16)] = 824,
  [SMALL_STATE(17)] = 883,
  [SMALL_STATE(18)] = 942,
  [SMALL_STATE(19)] = 1003,
  [SMALL_STATE(20)] = 1061,
  [SMALL_STATE(21)] = 1117,
  [SMALL_STATE(22)] = 1174,
  [SMALL_STATE(23)] = 1227,
  [SMALL_STATE(24)] = 1280,
  [SMALL_STATE(25)] = 1337,
  [SMALL_STATE(26)] = 1390,
  [SMALL_STATE(27)] = 1443,
  [SMALL_STATE(28)] = 1498,
  [SMALL_STATE(29)] = 1552,
  [SMALL_STATE(30)] = 1606,
  [SMALL_STATE(31)] = 1657,
  [SMALL_STATE(32)] = 1706,
  [SMALL_STATE(33)] = 1757,
  [SMALL_STATE(34)] = 1806,
  [SMALL_STATE(35)] = 1855,
  [SMALL_STATE(36)] = 1906,
  [SMALL_STATE(37)] = 1955,
  [SMALL_STATE(38)] = 2004,
  [SMALL_STATE(39)] = 2053,
  [SMALL_STATE(40)] = 2102,
  [SMALL_STATE(41)] = 2151,
  [SMALL_STATE(42)] = 2202,
  [SMALL_STATE(43)] = 2251,
  [SMALL_STATE(44)] = 2300,
  [SMALL_STATE(45)] = 2349,
  [SMALL_STATE(46)] = 2398,
  [SMALL_STATE(47)] = 2447,
  [SMALL_STATE(48)] = 2496,
  [SMALL_STATE(49)] = 2545,
  [SMALL_STATE(50)] = 2594,
  [SMALL_STATE(51)] = 2643,
  [SMALL_STATE(52)] = 2692,
  [SMALL_STATE(53)] = 2741,
  [SMALL_STATE(54)] = 2765,
  [SMALL_STATE(55)] = 2789,
  [SMALL_STATE(56)] = 2809,
  [SMALL_STATE(57)] = 2829,
  [SMALL_STATE(58)] = 2851,
  [SMALL_STATE(59)] = 2871,
  [SMALL_STATE(60)] = 2893,
  [SMALL_STATE(61)] = 2913,
  [SMALL_STATE(62)] = 2941,
  [SMALL_STATE(63)] = 2967,
  [SMALL_STATE(64)] = 2987,
  [SMALL_STATE(65)] = 3007,
  [SMALL_STATE(66)] = 3027,
  [SMALL_STATE(67)] = 3047,
  [SMALL_STATE(68)] = 3067,
  [SMALL_STATE(69)] = 3087,
  [SMALL_STATE(70)] = 3109,
  [SMALL_STATE(71)] = 3129,
  [SMALL_STATE(72)] = 3149,
  [SMALL_STATE(73)] = 3171,
  [SMALL_STATE(74)] = 3191,
  [SMALL_STATE(75)] = 3217,
  [SMALL_STATE(76)] = 3241,
  [SMALL_STATE(77)] = 3261,
  [SMALL_STATE(78)] = 3281,
  [SMALL_STATE(79)] = 3301,
  [SMALL_STATE(80)] = 3321,
  [SMALL_STATE(81)] = 3351,
  [SMALL_STATE(82)] = 3371,
  [SMALL_STATE(83)] = 3404,
  [SMALL_STATE(84)] = 3431,
  [SMALL_STATE(85)] = 3458,
  [SMALL_STATE(86)] = 3485,
  [SMALL_STATE(87)] = 3512,
  [SMALL_STATE(88)] = 3539,
  [SMALL_STATE(89)] = 3566,
  [SMALL_STATE(90)] = 3593,
  [SMALL_STATE(91)] = 3620,
  [SMALL_STATE(92)] = 3653,
  [SMALL_STATE(93)] = 3680,
  [SMALL_STATE(94)] = 3707,
  [SMALL_STATE(95)] = 3733,
  [SMALL_STATE(96)] = 3759,
  [SMALL_STATE(97)] = 3775,
  [SMALL_STATE(98)] = 3791,
  [SMALL_STATE(99)] = 3806,
  [SMALL_STATE(100)] = 3821,
  [SMALL_STATE(101)] = 3836,
  [SMALL_STATE(102)] = 3851,
  [SMALL_STATE(103)] = 3866,
  [SMALL_STATE(104)] = 3881,
  [SMALL_STATE(105)] = 3896,
  [SMALL_STATE(106)] = 3911,
  [SMALL_STATE(107)] = 3926,
  [SMALL_STATE(108)] = 3941,
  [SMALL_STATE(109)] = 3956,
  [SMALL_STATE(110)] = 3973,
  [SMALL_STATE(111)] = 3990,
  [SMALL_STATE(112)] = 4010,
  [SMALL_STATE(113)] = 4030,
  [SMALL_STATE(114)] = 4044,
  [SMALL_STATE(115)] = 4058,
  [SMALL_STATE(116)] = 4072,
  [SMALL_STATE(117)] = 4092,
  [SMALL_STATE(118)] = 4112,
  [SMALL_STATE(119)] = 4126,
  [SMALL_STATE(120)] = 4144,
  [SMALL_STATE(121)] = 4158,
  [SMALL_STATE(122)] = 4172,
  [SMALL_STATE(123)] = 4192,
  [SMALL_STATE(124)] = 4206,
  [SMALL_STATE(125)] = 4224,
  [SMALL_STATE(126)] = 4242,
  [SMALL_STATE(127)] = 4256,
  [SMALL_STATE(128)] = 4273,
  [SMALL_STATE(129)] = 4292,
  [SMALL_STATE(130)] = 4309,
  [SMALL_STATE(131)] = 4322,
  [SMALL_STATE(132)] = 4339,
  [SMALL_STATE(133)] = 4356,
  [SMALL_STATE(134)] = 4373,
  [SMALL_STATE(135)] = 4386,
  [SMALL_STATE(136)] = 4399,
  [SMALL_STATE(137)] = 4412,
  [SMALL_STATE(138)] = 4425,
  [SMALL_STATE(139)] = 4438,
  [SMALL_STATE(140)] = 4455,
  [SMALL_STATE(141)] = 4468,
  [SMALL_STATE(142)] = 4481,
  [SMALL_STATE(143)] = 4496,
  [SMALL_STATE(144)] = 4511,
  [SMALL_STATE(145)] = 4528,
  [SMALL_STATE(146)] = 4541,
  [SMALL_STATE(147)] = 4554,
  [SMALL_STATE(148)] = 4573,
  [SMALL_STATE(149)] = 4590,
  [SMALL_STATE(150)] = 4607,
  [SMALL_STATE(151)] = 4621,
  [SMALL_STATE(152)] = 4637,
  [SMALL_STATE(153)] = 4653,
  [SMALL_STATE(154)] = 4669,
  [SMALL_STATE(155)] = 4681,
  [SMALL_STATE(156)] = 4697,
  [SMALL_STATE(157)] = 4709,
  [SMALL_STATE(158)] = 4721,
  [SMALL_STATE(159)] = 4737,
  [SMALL_STATE(160)] = 4753,
  [SMALL_STATE(161)] = 4765,
  [SMALL_STATE(162)] = 4781,
  [SMALL_STATE(163)] = 4795,
  [SMALL_STATE(164)] = 4811,
  [SMALL_STATE(165)] = 4827,
  [SMALL_STATE(166)] = 4843,
  [SMALL_STATE(167)] = 4857,
  [SMALL_STATE(168)] = 4873,
  [SMALL_STATE(169)] = 4885,
  [SMALL_STATE(170)] = 4897,
  [SMALL_STATE(171)] = 4913,
  [SMALL_STATE(172)] = 4929,
  [SMALL_STATE(173)] = 4941,
  [SMALL_STATE(174)] = 4957,
  [SMALL_STATE(175)] = 4971,
  [SMALL_STATE(176)] = 4983,
  [SMALL_STATE(177)] = 4997,
  [SMALL_STATE(178)] = 5013,
  [SMALL_STATE(179)] = 5027,
  [SMALL_STATE(180)] = 5039,
  [SMALL_STATE(181)] = 5051,
  [SMALL_STATE(182)] = 5067,
  [SMALL_STATE(183)] = 5083,
  [SMALL_STATE(184)] = 5099,
  [SMALL_STATE(185)] = 5111,
  [SMALL_STATE(186)] = 5127,
  [SMALL_STATE(187)] = 5139,
  [SMALL_STATE(188)] = 5155,
  [SMALL_STATE(189)] = 5166,
  [SMALL_STATE(190)] = 5177,
  [SMALL_STATE(191)] = 5188,
  [SMALL_STATE(192)] = 5201,
  [SMALL_STATE(193)] = 5212,
  [SMALL_STATE(194)] = 5225,
  [SMALL_STATE(195)] = 5236,
  [SMALL_STATE(196)] = 5247,
  [SMALL_STATE(197)] = 5260,
  [SMALL_STATE(198)] = 5273,
  [SMALL_STATE(199)] = 5284,
  [SMALL_STATE(200)] = 5297,
  [SMALL_STATE(201)] = 5308,
  [SMALL_STATE(202)] = 5321,
  [SMALL_STATE(203)] = 5332,
  [SMALL_STATE(204)] = 5341,
  [SMALL_STATE(205)] = 5354,
  [SMALL_STATE(206)] = 5365,
  [SMALL_STATE(207)] = 5376,
  [SMALL_STATE(208)] = 5387,
  [SMALL_STATE(209)] = 5398,
  [SMALL_STATE(210)] = 5411,
  [SMALL_STATE(211)] = 5424,
  [SMALL_STATE(212)] = 5435,
  [SMALL_STATE(213)] = 5448,
  [SMALL_STATE(214)] = 5459,
  [SMALL_STATE(215)] = 5472,
  [SMALL_STATE(216)] = 5485,
  [SMALL_STATE(217)] = 5496,
  [SMALL_STATE(218)] = 5504,
  [SMALL_STATE(219)] = 5512,
  [SMALL_STATE(220)] = 5520,
  [SMALL_STATE(221)] = 5530,
  [SMALL_STATE(222)] = 5540,
  [SMALL_STATE(223)] = 5550,
  [SMALL_STATE(224)] = 5558,
  [SMALL_STATE(225)] = 5568,
  [SMALL_STATE(226)] = 5578,
  [SMALL_STATE(227)] = 5588,
  [SMALL_STATE(228)] = 5598,
  [SMALL_STATE(229)] = 5608,
  [SMALL_STATE(230)] = 5616,
  [SMALL_STATE(231)] = 5626,
  [SMALL_STATE(232)] = 5633,
  [SMALL_STATE(233)] = 5640,
  [SMALL_STATE(234)] = 5647,
  [SMALL_STATE(235)] = 5654,
  [SMALL_STATE(236)] = 5661,
  [SMALL_STATE(237)] = 5668,
  [SMALL_STATE(238)] = 5675,
  [SMALL_STATE(239)] = 5682,
  [SMALL_STATE(240)] = 5689,
  [SMALL_STATE(241)] = 5696,
  [SMALL_STATE(242)] = 5703,
  [SMALL_STATE(243)] = 5710,
  [SMALL_STATE(244)] = 5717,
  [SMALL_STATE(245)] = 5724,
  [SMALL_STATE(246)] = 5731,
  [SMALL_STATE(247)] = 5738,
  [SMALL_STATE(248)] = 5745,
  [SMALL_STATE(249)] = 5752,
  [SMALL_STATE(250)] = 5759,
  [SMALL_STATE(251)] = 5766,
  [SMALL_STATE(252)] = 5773,
  [SMALL_STATE(253)] = 5780,
  [SMALL_STATE(254)] = 5787,
  [SMALL_STATE(255)] = 5794,
  [SMALL_STATE(256)] = 5801,
  [SMALL_STATE(257)] = 5808,
  [SMALL_STATE(258)] = 5815,
  [SMALL_STATE(259)] = 5822,
  [SMALL_STATE(260)] = 5829,
  [SMALL_STATE(261)] = 5836,
  [SMALL_STATE(262)] = 5843,
  [SMALL_STATE(263)] = 5850,
  [SMALL_STATE(264)] = 5857,
  [SMALL_STATE(265)] = 5864,
  [SMALL_STATE(266)] = 5871,
  [SMALL_STATE(267)] = 5878,
  [SMALL_STATE(268)] = 5885,
  [SMALL_STATE(269)] = 5892,
  [SMALL_STATE(270)] = 5899,
  [SMALL_STATE(271)] = 5906,
  [SMALL_STATE(272)] = 5913,
  [SMALL_STATE(273)] = 5920,
  [SMALL_STATE(274)] = 5927,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(116),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(259),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(100),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(104),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(105),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(243),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(183),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(253),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(271),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(134),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(97),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [41] = {.entry = {.count = 1, .reusable = false}}, SHIFT(96),
  [43] = {.entry = {.count = 1, .reusable = false}}, SHIFT(266),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(182),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(246),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [55] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [57] = {.entry = {.count = 1, .reusable = false}}, SHIFT(135),
  [59] = {.entry = {.count = 1, .reusable = false}}, SHIFT(182),
  [61] = {.entry = {.count = 1, .reusable = false}}, SHIFT(246),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(91),
  [65] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anonymous_field, 1, 0, 0),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anonymous_field, 1, 0, 0),
  [69] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [79] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(123),
  [83] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binding_ref, 1, 0, 0),
  [87] = {.entry = {.count = 1, .reusable = false}}, SHIFT(226),
  [89] = {.entry = {.count = 1, .reusable = false}}, SHIFT(147),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding_ref, 1, 0, 0),
  [93] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [95] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_binding_ref, 1, 0, 0), SHIFT(96),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [100] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_binding_ref, 1, 0, 0), SHIFT(19),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(186),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [109] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(263),
  [113] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [115] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [117] = {.entry = {.count = 1, .reusable = false}}, SHIFT(66),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(67),
  [121] = {.entry = {.count = 1, .reusable = false}}, SHIFT(79),
  [123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [127] = {.entry = {.count = 1, .reusable = false}}, SHIFT(273),
  [129] = {.entry = {.count = 1, .reusable = false}}, SHIFT(267),
  [131] = {.entry = {.count = 1, .reusable = false}}, SHIFT(70),
  [133] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [135] = {.entry = {.count = 1, .reusable = false}}, SHIFT(81),
  [137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [139] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [141] = {.entry = {.count = 1, .reusable = false}}, SHIFT(261),
  [143] = {.entry = {.count = 1, .reusable = false}}, SHIFT(262),
  [145] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assoc_list, 4, 0, 0),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assoc_list, 4, 0, 0),
  [149] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_assoc_list, 3, 0, 0),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assoc_list, 3, 0, 0),
  [153] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_call, 4, 0, 0),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_call, 4, 0, 0),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_expr, 1, 0, 0),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_expr, 1, 0, 0),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_not, 2, 0, 0),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_not, 2, 0, 0),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(227),
  [167] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_paren, 3, 0, 0),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_paren, 3, 0, 0),
  [171] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_comparison, 3, 0, 0),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_comparison, 3, 0, 0),
  [175] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_field_access, 3, 0, 0),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_field_access, 3, 0, 0),
  [179] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_or, 3, 0, 0),
  [181] = {.entry = {.count = 1, .reusable = false}}, SHIFT(34),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_or, 3, 0, 0),
  [185] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [189] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_and, 3, 0, 0),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_and, 3, 0, 0),
  [193] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_call, 6, 0, 0),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_call, 6, 0, 0),
  [197] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_call, 8, 0, 0),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_call, 8, 0, 0),
  [201] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_var, 1, 0, 0),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_var, 1, 0, 0),
  [205] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_constraint_literal, 1, 0, 0),
  [207] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_literal, 1, 0, 0),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(224),
  [211] = {.entry = {.count = 1, .reusable = false}}, SHIFT(42),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [215] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [221] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_annotation, 3, 0, 0),
  [223] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_annotation, 3, 0, 0),
  [225] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [227] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [229] = {.entry = {.count = 1, .reusable = true}}, SHIFT(274),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(98),
  [233] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(270),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(251),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(249),
  [257] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [259] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [263] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [265] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(2),
  [268] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(116),
  [271] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(259),
  [274] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [276] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_base_type, 1, 0, 0),
  [278] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_base_type, 1, 0, 0),
  [280] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_type, 2, 0, 0),
  [282] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_type, 2, 0, 0),
  [284] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_reference_type, 2, 0, 0),
  [286] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_reference_type, 2, 0, 0),
  [288] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_named_type, 1, 0, 0),
  [290] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_named_type, 1, 0, 0),
  [292] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_concrete_type, 4, 0, 0),
  [294] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_concrete_type, 4, 0, 0),
  [296] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal_type, 1, 0, 0),
  [298] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal_type, 1, 0, 0),
  [300] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_type, 4, 0, 0),
  [302] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_type, 4, 0, 0),
  [304] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_type, 3, 0, 0),
  [306] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_type, 3, 0, 0),
  [308] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_type, 3, 0, 0),
  [310] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_type, 3, 0, 0),
  [312] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_expr, 1, 0, 0),
  [314] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type_expr, 1, 0, 0),
  [316] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_intersection_type, 3, 0, 0),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [320] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_intersection_type, 3, 0, 0),
  [322] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_union_type, 3, 0, 0),
  [324] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_union_type, 3, 0, 0),
  [326] = {.entry = {.count = 1, .reusable = false}}, SHIFT(208),
  [328] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field, 2, 0, 9),
  [330] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 2, 0, 9),
  [332] = {.entry = {.count = 1, .reusable = false}}, SHIFT(191),
  [334] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field, 3, 0, 11),
  [336] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 3, 0, 11),
  [338] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_value, 3, 0, 0),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_value, 3, 0, 0),
  [342] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_value, 3, 0, 0),
  [344] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_value, 3, 0, 0),
  [346] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value, 1, 0, 0),
  [348] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value, 1, 0, 0),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(217),
  [352] = {.entry = {.count = 1, .reusable = true}}, SHIFT(242),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(250),
  [356] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [358] = {.entry = {.count = 1, .reusable = true}}, SHIFT(232),
  [360] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field, 4, 0, 13),
  [362] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 4, 0, 13),
  [364] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variant_value, 2, 0, 0),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variant_value, 2, 0, 0),
  [368] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_source_path_repeat1, 2, 0, 0),
  [370] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_path_repeat1, 2, 0, 0),
  [372] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_path_repeat1, 2, 0, 0), SHIFT_REPEAT(247),
  [375] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_value, 2, 0, 0),
  [377] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_value, 2, 0, 0),
  [379] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_value, 2, 0, 0),
  [381] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_value, 2, 0, 0),
  [383] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field, 5, 0, 14),
  [385] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 5, 0, 14),
  [387] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal_value, 1, 0, 0),
  [389] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal_value, 1, 0, 0),
  [391] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, 0, 2),
  [393] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [395] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3, 0, 2),
  [397] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2, 0, 1),
  [399] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2, 0, 1),
  [401] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type_ref, 1, 0, 0),
  [403] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_ref, 1, 0, 0),
  [405] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [407] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [409] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [411] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [413] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [415] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ilk_field, 4, 0, 8),
  [417] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ilk_field, 4, 0, 8),
  [419] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [421] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_source_path, 1, 0, 0),
  [423] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_path, 1, 0, 0),
  [425] = {.entry = {.count = 1, .reusable = false}}, SHIFT(247),
  [427] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_origin_path, 1, 0, 0),
  [429] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_origin_path, 1, 0, 0),
  [431] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_origin_path, 2, 0, 0),
  [433] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_origin_path, 2, 0, 0),
  [435] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ilk_field, 3, 0, 5),
  [437] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ilk_field, 3, 0, 5),
  [439] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ilk_field, 3, 0, 6),
  [441] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ilk_field, 3, 0, 6),
  [443] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [445] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_source_path, 2, 0, 0),
  [447] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_path, 2, 0, 0),
  [449] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ilk_field, 2, 0, 3),
  [451] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ilk_field, 2, 0, 3),
  [453] = {.entry = {.count = 1, .reusable = true}}, SHIFT(257),
  [455] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_list_elements_repeat1, 2, 0, 0),
  [457] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_list_elements_repeat1, 2, 0, 0), SHIFT_REPEAT(27),
  [460] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_elements_repeat1, 2, 0, 0), SHIFT_REPEAT(27),
  [463] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_value_field_list_repeat1, 2, 0, 0),
  [465] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_value_field_list_repeat1, 2, 0, 0), SHIFT_REPEAT(209),
  [468] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_value_field_list_repeat1, 2, 0, 0), SHIFT_REPEAT(209),
  [471] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_source_args, 1, 0, 0),
  [473] = {.entry = {.count = 1, .reusable = false}}, SHIFT(225),
  [475] = {.entry = {.count = 1, .reusable = true}}, SHIFT(225),
  [477] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding, 7, 0, 12),
  [479] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binding, 7, 0, 12),
  [481] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anonymous_fields, 2, 0, 0),
  [483] = {.entry = {.count = 1, .reusable = false}}, SHIFT(228),
  [485] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [487] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_elements, 2, 0, 0),
  [489] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [491] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [493] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anonymous_fields, 1, 0, 0),
  [495] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ilk_field_list, 1, 0, 0),
  [497] = {.entry = {.count = 1, .reusable = false}}, SHIFT(127),
  [499] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [501] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [503] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ilk_field_list, 2, 0, 0),
  [505] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field_list, 2, 0, 0),
  [507] = {.entry = {.count = 1, .reusable = false}}, SHIFT(209),
  [509] = {.entry = {.count = 1, .reusable = true}}, SHIFT(209),
  [511] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_annotation_args_repeat1, 2, 0, 0),
  [513] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_annotation_args_repeat1, 2, 0, 0), SHIFT_REPEAT(241),
  [516] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_annotation_args_repeat1, 2, 0, 0), SHIFT_REPEAT(241),
  [519] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [521] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_ilk_field_list_repeat1, 2, 0, 0),
  [523] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_ilk_field_list_repeat1, 2, 0, 0), SHIFT_REPEAT(127),
  [526] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ilk_field_list_repeat1, 2, 0, 0), SHIFT_REPEAT(127),
  [529] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding, 4, 0, 4),
  [531] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binding, 4, 0, 4),
  [533] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_source_args_repeat1, 2, 0, 0),
  [535] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_args_repeat1, 2, 0, 0), SHIFT_REPEAT(225),
  [538] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_args_repeat1, 2, 0, 0), SHIFT_REPEAT(225),
  [541] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_annotation_args, 1, 0, 0),
  [543] = {.entry = {.count = 1, .reusable = false}}, SHIFT(241),
  [545] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [547] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_elements, 1, 0, 0),
  [549] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ilk_field_repeat1, 2, 0, 0),
  [551] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_ilk_field_repeat1, 2, 0, 0), SHIFT_REPEAT(116),
  [554] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_path_repeat1, 2, 0, 0), SHIFT_REPEAT(257),
  [557] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field_list, 1, 0, 0),
  [559] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding, 5, 0, 7),
  [561] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binding, 5, 0, 7),
  [563] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_annotation_args, 2, 0, 0),
  [565] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [567] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [569] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [571] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [573] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding, 6, 0, 10),
  [575] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binding, 6, 0, 10),
  [577] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_source_args, 2, 0, 0),
  [579] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_anonymous_fields_repeat1, 2, 0, 0),
  [581] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_anonymous_fields_repeat1, 2, 0, 0), SHIFT_REPEAT(228),
  [584] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_anonymous_fields_repeat1, 2, 0, 0), SHIFT_REPEAT(228),
  [587] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_anonymous_field, 2, 0, 0),
  [589] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_anonymous_field, 2, 0, 0),
  [591] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_annotation_args_repeat1, 2, 0, 0),
  [593] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_anonymous_fields_repeat1, 2, 0, 0),
  [595] = {.entry = {.count = 1, .reusable = false}}, SHIFT(132),
  [597] = {.entry = {.count = 1, .reusable = false}}, SHIFT(237),
  [599] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_ilk_field_list_repeat1, 2, 0, 0),
  [601] = {.entry = {.count = 1, .reusable = true}}, SHIFT(178),
  [603] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field, 5, 0, 13),
  [605] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 5, 0, 13),
  [607] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_args_repeat1, 2, 0, 0),
  [609] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [611] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [613] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_origin_paths, 1, 0, 0),
  [615] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field, 6, 0, 14),
  [617] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 6, 0, 14),
  [619] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [621] = {.entry = {.count = 1, .reusable = true}}, SHIFT(252),
  [623] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list_element, 1, 0, 0),
  [625] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list_element, 1, 0, 0),
  [627] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_origin_paths, 2, 0, 0),
  [629] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_origin, 5, 0, 0),
  [631] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_origin, 5, 0, 0),
  [633] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_origin_paths_repeat1, 2, 0, 0), SHIFT_REPEAT(222),
  [636] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_origin_paths_repeat1, 2, 0, 0),
  [638] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field, 3, 0, 9),
  [640] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 3, 0, 9),
  [642] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_value_field_list_repeat1, 2, 0, 0),
  [644] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_list_elements_repeat1, 2, 0, 0),
  [646] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_origin, 1, 0, 0),
  [648] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_origin, 1, 0, 0),
  [650] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [652] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_value_field, 4, 0, 11),
  [654] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_value_field, 4, 0, 11),
  [656] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [658] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_origin, 2, 0, 0),
  [660] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_origin, 2, 0, 0),
  [662] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [664] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_assoc_list_repeat1, 2, 0, 0),
  [666] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_assoc_list_repeat1, 2, 0, 0), SHIFT_REPEAT(252),
  [669] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_refinement, 3, 0, 0),
  [671] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_refinement, 3, 0, 0),
  [673] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_annotation, 2, 0, 0),
  [675] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_annotation, 5, 0, 0),
  [677] = {.entry = {.count = 1, .reusable = true}}, SHIFT(231),
  [679] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cardinality, 2, 0, 0),
  [681] = {.entry = {.count = 1, .reusable = true}}, SHIFT(171),
  [683] = {.entry = {.count = 1, .reusable = false}}, SHIFT(73),
  [685] = {.entry = {.count = 1, .reusable = false}}, SHIFT(268),
  [687] = {.entry = {.count = 1, .reusable = false}}, SHIFT(60),
  [689] = {.entry = {.count = 1, .reusable = false}}, SHIFT(254),
  [691] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [693] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cardinality, 1, 0, 0),
  [695] = {.entry = {.count = 1, .reusable = true}}, SHIFT(220),
  [697] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cardinality, 3, 0, 0),
  [699] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [701] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [703] = {.entry = {.count = 1, .reusable = true}}, SHIFT(160),
  [705] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [707] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [709] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [711] = {.entry = {.count = 1, .reusable = true}}, SHIFT(218),
  [713] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [715] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [717] = {.entry = {.count = 1, .reusable = true}}, SHIFT(221),
  [719] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [721] = {.entry = {.count = 1, .reusable = true}}, SHIFT(272),
  [723] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [725] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [727] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [729] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [731] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [733] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [735] = {.entry = {.count = 1, .reusable = true}}, SHIFT(258),
  [737] = {.entry = {.count = 1, .reusable = true}}, SHIFT(223),
  [739] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [741] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [743] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [745] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [747] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [749] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [751] = {.entry = {.count = 1, .reusable = true}}, SHIFT(244),
  [753] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [755] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [757] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [759] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [761] = {.entry = {.count = 1, .reusable = true}}, SHIFT(202),
  [763] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [765] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [767] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [769] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [771] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [773] = {.entry = {.count = 1, .reusable = true}}, SHIFT(269),
  [775] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [777] = {.entry = {.count = 1, .reusable = true}}, SHIFT(245),
  [779] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [781] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
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
