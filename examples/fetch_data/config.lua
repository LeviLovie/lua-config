function Config()
	print(fetch_data("https://yesno.wtf/api")["response"])

	local config = {
		width = 800,
	}
	config.height = config.width * 3 / 4

	return config
end
