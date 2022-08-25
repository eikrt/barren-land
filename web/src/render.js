import * as THREE from 'three'
const loader = new THREE.TextureLoader()

export const tileColors = {
  grass: 0x77d182,
  sand: 0xe6e6a5,
  barren_land: 0xe6e6a5,
  dune_sand: 0xe6e6a5,
  salt: 0xffffff,
  ice: 0xfffff,
  ash: 0x525245,
  gravel: 0x9ca6a2,
  ruins: 0x9ca6a2,
  water: 0x3c82e6,
  shipwreck: 0x9ca6a2,
  unknown: 0x000000
}

export const tileTextures = {
  barren_land: loader.load(
    'res/fine_sand.png'
  ),
  dune_sand: loader.load(
    'res/fine_sand.png'
  ),
  sand: loader.load(
    'res/fine_sand.png'
  ),
  ice: loader.load(
    'res/ice.png'
  ),
  ash: loader.load(
    'res/ash.png'
  ),
  gravel: loader.load(
    'res/fine_sand.png'
  ),
  ruins: loader.load(
    'res/fine_sand.png'
  ),
  water: loader.load(
    'res/water.png'
  ),
  shipwreck: loader.load(
    'res/fine_sand.png'
  ),
  grass: loader.load(
    'res/grass.png'
  ),
  salt: loader.load(
    'res/fine_sand.png'
  ),
  unknown: loader.load(
    'res/fine_sand.png'
  )
}

export default class Renderer {
    constructor(el, camera, scene, worldProperties) {
	this.rend = null
	this.chunkViewRange = 2
	this.tileRatio = 200
	this.initRender(el)
	this.camera = camera
	this.scene = scene
	this.worldProperties = worldProperties
    }
renderScene() {
   this.rend && this.rend.render(this.scene, this.camera)
}
initRender (el) {

  this.rend = new THREE.WebGLRenderer({ antialias: true, canvas: el })
  this.rend.setSize(window.innerWidth, window.innerHeight)
  this.rend.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  this.rend.shadowMap.enabled = true
  this.rend.shadowMap.type = THREE.PCFSoftShadowMap

}
}
