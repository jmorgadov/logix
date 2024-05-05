require("flutter-tools").setup_project({
	{
		name = "dev", -- an arbitrary name that you provide so you can recognise this config
		-- target = "./flutter_logix_gui/lib/main.dart", -- your target
		device = "linux", -- the device ID, which you can get by running `flutter devices`
		flutter_mode = "debug", -- possible values: `debug`, `profile` or `release`, defaults to `debug`
	},
	{
		name = "release", -- an arbitrary name that you provide so you can recognise this config
		-- target = "./flutter_logix_gui/lib/main.dart", -- your target
		device = "linux", -- the device ID, which you can get by running `flutter devices`
		flutter_mode = "release", -- possible values: `debug`, `profile` or `release`, defaults to `debug`
	},
})
