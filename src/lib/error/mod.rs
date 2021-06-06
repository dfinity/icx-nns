// The type to represent NNS CLI results.
pub type NnsCliResult<T = ()> = anyhow::Result<T>;

// The type to represent NNS CLI errors.
pub type NnsCliError = anyhow::Error;
