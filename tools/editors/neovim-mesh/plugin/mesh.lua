if vim.g.loaded_mesh_nvim_plugin == 1 then
  return
end
vim.g.loaded_mesh_nvim_plugin = 1

local ok, mesh = pcall(require, 'mesh')
if not ok then
  vim.schedule(function()
    vim.notify(string.format('mesh.nvim bootstrap failed to load: %s', mesh), vim.log.levels.ERROR, { title = 'mesh.nvim' })
  end)
  return
end

local supported, support_error = mesh.assert_native_lsp()
if not supported then
  mesh.set_last_error(support_error)
  vim.schedule(function()
    vim.notify(support_error, vim.log.levels.WARN, { title = 'mesh.nvim' })
  end)
  return
end

local ok_enable, enable_error = pcall(vim.lsp.enable, 'mesh')
if not ok_enable then
  local message = string.format('mesh.nvim failed to enable native LSP config: %s', tostring(enable_error))
  mesh.set_last_error(message)
  vim.schedule(function()
    vim.notify(message, vim.log.levels.ERROR, { title = 'mesh.nvim' })
  end)
end
