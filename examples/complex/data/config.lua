function Config()
	local config = {
		window = {
			width = 800,
			title = "The best game ever",
		},
		graphics = {
			x = 75,
			y = 25,
			width = 50,
			height = 10,
		},
	}
	config["window"]["height"] = config["window"]["width"] * 3 / 4

	return config
end
