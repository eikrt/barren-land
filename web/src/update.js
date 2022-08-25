import Network from './network'
import {Render, tileTextures, tileColors} from './render'

import * as THREE from 'three'
const network = new Network()
export default class Update {
    constructor(scene, camera, worldProperties) {
	this.scene = scene
	this.camera = camera
	this.worldProperties = worldProperties
	this.chunkViewRange = 2
	this.tileRatio = 200
	
	this.tileObjects = [] 
	this.entityObjects = [] 
	this.clearEntityChange = 0
	this.clearEntityTime = 1000
    }
    
async updateTilesInRange() {
    const cameraX = this.camera.position.x / this.worldProperties.chunk_size;
    const cameraY = this.camera.position.y / this.worldProperties.chunk_size;
      for (let i = Math.round(cameraY - this.chunkViewRange); i < Math.round(cameraY + this.chunkViewRange); i++) {
        for (let j = Math.round(cameraX - this.chunkViewRange); j < Math.round(cameraX + this.chunkViewRange); j++) {
            if (j < 0 || i < 0 || j > this.worldProperties.world_width || i > this.worldProperties.world_height) {
                continue
            }
            this.updateTiles(j,i)
      } 
  }
}
async updateEntities(x,y) {
  this.clearEntities() 
  const chunkEntities = await network.getChunkEntities(x, y)
    Object.entries(chunkEntities).forEach(([key, value]) => {
	this.addEntity({key: value})
    });
}
async updateTiles(x,y) {
    
  const chunkTiles = await network.getChunkTiles(x, y)
    for (let i = 0; i < this.worldProperties.chunk_size; i++) {
	for (let j = 0; j < this.worldProperties.chunk_size; j++) {
	    const texture = tileTextures[chunkTiles[j][i].tile_type]
	    this.addTile(chunkTiles[j][i],texture)
	}
    }
}
async updateEntitiesInRange() {
    const cameraX = this.camera.position.x / this.worldProperties.chunk_size;
    const cameraY = this.camera.position.y / this.worldProperties.chunk_size;
      for (let i = Math.round(cameraY - this.chunkViewRange); i < Math.round(cameraY + this.chunkViewRange); i++) {
        for (let j = Math.round(cameraX - this.chunkViewRange); j < Math.round(cameraX + this.chunkViewRange); j++) {
            if (j < 0 || i < 0 || j > this.worldProperties.world_width || i > this.worldProperties.world_height) {
                continue
            }
            this.updateEntities(j,i)
      } 
  }
}
addTile (tile, cubeTexture) {
  const geometry = new THREE.BoxGeometry(1, 1, tile.h / this.tileRatio)

  let color = 0xff0000
  if (tile.tile_type in tileColors) {
    color = tileColors[tile.tile_type]
  } else {
    color = tileColors.unknown
  }
  const material = new THREE.MeshStandardMaterial({ color, map: cubeTexture })
  const mesh = new THREE.Mesh(geometry, material)
  mesh.position.set(tile.x, tile.y, 0)
  mesh.castShadow = true 
  mesh.receiveShadow = true 
    
  this.tileObjects.push(mesh)
  this.scene.add(mesh)
}
addEntity (entityPair) {
  const entity = Object.values(entityPair)[0]
  const geometry = new THREE.BoxGeometry(1, 0.1, 1)

  let color = 0xff0000
  const material = new THREE.MeshStandardMaterial({ color: 0xffffff })
  const mesh = new THREE.Mesh(geometry, material)
  console.log(entity.standing_tile)
  mesh.position.set(entity.x, entity.y, entity.standing_tile.h / this.tileRatio + 1)
  mesh.castShadow = true 
  mesh.receiveShadow = true 
  this.entityObjects.push(mesh)
  this.scene.add(mesh)
}
async clearTiles() {
 this.tileObjects.forEach(c => { 
    this.scene.remove(c) 
})
}
async clearEntities() {
 const clearedEntities = this.entityObjects.map(eo => eo)
 setTimeout(() => {
 clearedEntities.forEach(c => { 
    this.scene.remove(c) 
 })
}, 100)
}
    
refreshChunk(vertical) {
    if (this.camera.position.x % this.worldProperties.chunk_size == 0 && !vertical) {

            this.clearTiles()
            this.updateTilesInRange()
    }
        if ( this.camera.position.y % this.worldProperties.chunk_size == 0 && vertical) {
            this.clearTiles()
            this.updateTilesInRange()

        }
}
refreshEntities() {
    this.clearEntityChange += 10
    if (this.clearEntityChange > this.clearEntityTime) {
        this.clearEntities()
	this.clearEntityChange = 0
        this.updateEntitiesInRange()
	
    }
}
}
