function Config()
	local config = {
		width = 800,
		value_not_present_in_default_config = 1,
	}
	config.height = config.width * 3 / 4

	return config
end
