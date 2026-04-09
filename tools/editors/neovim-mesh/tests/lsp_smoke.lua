vim.g.mesh_smoke_phase = 'lsp'
dofile(vim.fs.joinpath(assert(os.getenv('MESH_REPO_ROOT'), 'MESH_REPO_ROOT is required'), 'tools', 'editors', 'neovim-mesh', 'tests', 'smoke.lua'))
