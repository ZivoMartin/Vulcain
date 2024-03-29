
const { contextBridge, ipcRenderer } = require('electron')

contextBridge.exposeInMainWorld('message', {
  save: (path, new_txt) => {
    return ipcRenderer.invoke('save', path, new_txt)
  },
  get_content: (path) => ipcRenderer.invoke('get_content', path),
  run: (path) => ipcRenderer.invoke('run', path),
  openide: () => ipcRenderer.invoke('openide'),
  loadFile: (file) => ipcRenderer.invoke("loadFile", file),
  addProject: (name) => ipcRenderer.invoke('addProject', name),
  init: () => ipcRenderer.invoke("init"),
  getProjects: async () => {
    return await ipcRenderer.invoke("getProjects");
  },
  openProject: (name) => ipcRenderer.invoke("openProject", name),
  getCurrentProjectData: () => {
    return ipcRenderer.invoke("getCurrentProjectData")
  },
  addFile: (name) => ipcRenderer.invoke("addFile", name),
  backToProject: () => ipcRenderer.invoke("backToProject"),
  getTabText: (tabName) => ipcRenderer.invoke("getTabText", tabName),
})
