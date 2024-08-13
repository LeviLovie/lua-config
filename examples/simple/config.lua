function Config()
	local data = fetch_data()
	print(data["value"])

	local config = {
		width = 800,
	}
	config.height = config.width * 3 / 4

	return config
end
