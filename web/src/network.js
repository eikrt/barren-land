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
async addPlayer(worldProperties,x,y,name,id) {
    
    const response = await axios.post("http://localhost:8081/queue", {
	params: {command: "spawn",
	id: "0",
	x: x.toString(),
	y: y.toString(),
	chunk_x: (x / worldProperties.chunk_size).toString(),
	chunk_y: (y / worldProperties.chunk_size).toString(),
	name: name}
    })
}
}
