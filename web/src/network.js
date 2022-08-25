import axios from 'axios'
export default class Network {
    constructor() {
    }
    
async getChunkTiles (x, y) {
  const response = await axios.get(`http://localhost:8081/tiles/${x}/${y}`)
  return response.data.tiles
}
async getChunkEntities (x, y) {
  const response = await axios.get(`http://localhost:8081/entities/${x}/${y}`)
  return response.data.entities
}
async getWorldProperties () {
  const response = await axios.get('http://localhost:8081/world_properties')
  return response.data
}
async addPlayer() {
    
    const response = await axios.post("http://localhost:8081/queue", {
	params: {command: "spawn",
	id: "0",
	x: "0",
	y: "0",
	chunk_x: "0",
	chunk_y: "0",
	name: "web"}
    })
}
}
