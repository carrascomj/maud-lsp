-- Put this somewhere in your nvim config (init.lua, etc.)
local util = require 'lspconfig.util'

local lspconfig = require'lspconfig'
local configs = require'lspconfig.configs'
if not configs.maud then
  configs.maud = {
  default_config = {
    filetypes = { 'toml', 'csv' },
    -- Should be in your path
    cmd = {'maud-lsp'},
    root_dir = function(fname)
      local root = util.root_pattern(unpack({"config.toml"}))(fname)
      return root
    end,
    single_file_support = true,
  },
  docs = {
    description = [[
		Maud stuff
]],
  },
  };
end
lspconfig.maud.setup{}
