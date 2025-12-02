package app

import "core:encoding/json"
import "core:io"
import "core:os"

FILE :: "pingconfig.json"

Config :: struct {
	naam:          cstring,
	ping_interval: u64,
}

ConfigError :: union #shared_nil {
	os.Error,
	json.Unmarshal_Error,
	json.Marshal_Error,
}


config_exists :: proc() -> bool {
	return os.is_file(FILE)
}

create_default_config :: proc() -> (config: Config, err: ConfigError) {
	config = Config {
		naam          = "window naam",
		ping_interval = 30,
	}

	fp := os.open(FILE, os.O_WRONLY | os.O_CREATE) or_return
	defer os.close(fp)

	stream := os.stream_from_handle(fp)
	writer := io.to_writer(stream)


	options := json.Marshal_Options {
		pretty = true,
	}

	err = json.marshal_to_writer(writer, config, &options)

	return
}

config_read :: proc() -> (conf: Config, err: ConfigError) {
	data := os.read_entire_file_or_err(FILE) or_return
	json.unmarshal(data, &conf) or_return


	return
}

