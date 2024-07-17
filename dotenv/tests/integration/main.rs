/// `dotenvy` integration tests
mod api {
    mod dotenv;
    mod dotenv_iter;
    mod dotenv_override;
    mod from_filename;
    mod from_filename_iter;
    mod from_filename_override;
    mod from_path;
    mod from_path_iter;
    mod from_path_override;
    mod from_read;
    mod from_read_iter;
    mod from_read_override;
    mod var;
    mod vars;
}

/// different environment-setup test-cases
mod case {
    mod bom;
    mod comment;
    mod directory;
    mod envfile;
    mod export;
    mod multiline;
    mod multiline_comment;
    mod quote;
    mod var_substitution;
    mod whitespace;
}

/// constructors, helpers, and assertions
pub(crate) mod util;
