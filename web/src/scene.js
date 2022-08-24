import * as THREE from 'three'
import { PointerLockControls } from './PointerLockControls'
import axios from 'axios'
const scene = new THREE.Scene()
const chunkViewRange = 2
const tileColors = {
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

const loader = new THREE.TextureLoader()
let worldProperties = null
const tiles = null
const entities = null
const tileObjects = [] 
const entityObjects = [] 
let renderer = null
let controls = null
const clearEntityTime = 1000;
let clearEntityChange = 0;
const tileTextures = {
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
async function getChunkTiles (x, y) {
  const response = await axios.get(`http://localhost:8081/tiles/${x}/${y}`)
  return response.data.tiles
}
async function getChunkEntities (x, y) {
  const response = await axios.get(`http://localhost:8081/entities/${x}/${y}`)
  return response.data.entities
}
async function getWorldProperties () {
  const response = await axios.get('http://localhost:8081/world_properties')
  return response.data
}

const camera = new THREE.PerspectiveCamera(

  75,
  window.innerWidth / window.innerHeight,
  0.01,
  250

)
camera.up = new THREE.Vector3(0, 0, 1)
camera.rotation.y = 0// 3.14/4;
camera.rotation.x = 3.14 / 4
camera.position.x = 1.0
camera.position.y = 1.0
camera.position.z = 4.0
const light = new THREE.DirectionalLight(0xffffff, 1)

const tick = () => {
  setTimeout(() => {
  if (renderer) {
    renderer.render(scene, camera)
  }
  if (controls) {
  }
  light.target.updateMatrixWorld()

    refreshEntities()
  window.requestAnimationFrame(tick)

  }, 10)}

const resize = () => {
  renderer.setSize(window.innerWidth, window.innerHeight)
  camera.aspect = window.innerWidth / window.innerHeight
  camera.updateProjectionMatrix()
}

const onKeyDown = function (event) {
  switch (event.code) {
    case 'KeyW':
      refreshChunk(true)
      controls.moveForward(1)
      break
    case 'KeyA':
      refreshChunk(false)
      controls.moveRight(-1)
      break
    case 'KeyS':
      refreshChunk(true)
      controls.moveForward(-1)
      break
    case 'KeyD':
      refreshChunk(false)
      controls.moveRight(1)
      break
  }
}
function refreshChunk(vertical) {
    if (camera.position.x % worldProperties.chunk_size == 0 && !vertical) {

            clearTiles()
            updateTilesInRange()
    }
        if ( camera.position.y % worldProperties.chunk_size == 0 && vertical) {
            clearTiles()
            updateTilesInRange()

        }
}
function refreshEntities() {
    clearEntityChange += 10
    if (clearEntityChange > clearEntityTime) {
        clearEntities()
	clearEntityChange = 0
        updateEntitiesInRange()
    }
}
function addTile (tile, cubeTexture) {
  const geometry = new THREE.BoxGeometry(1, 1, tile.h / 200)

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
    
  tileObjects.push(mesh)
  scene.add(mesh)
}
function addEntity (entityPair) {
  const entity = Object.values(entityPair)[0]
  const geometry = new THREE.BoxGeometry(10, 10, 10)

  let color = 0xff0000
  const material = new THREE.MeshStandardMaterial({ color: 0xffffff })
  const mesh = new THREE.Mesh(geometry, material)
  mesh.position.set(entity.x, entity.y, 0)
  mesh.castShadow = true 
  mesh.receiveShadow = true 
  entityObjects.push(mesh)
//  scene.add(mesh)
}
async function clearTiles() {
 tileObjects.forEach(c => { 
    scene.remove(c) 
})
}
async function clearEntities() {
 const clearedEntities = entityObjects.map(eo => eo)
 setTimeout(() => {
 clearedEntities.forEach(c => { 
    scene.remove(c) 
 })
}, 100)
}
async function initScene () {
  light.position.set(-12, -12, 32) 
  light.castShadow = true 
  scene.add(light)
  scene.add(light.target)

  light.shadow.mapSize.width = 512
  light.shadow.mapSize.height = 512
  light.shadow.camera.near = 0.5
  light.shadow.camera.far = 1000
  light.intensity = 1
  worldProperties = await getWorldProperties()

    updateTilesInRange()
}
async function updateTilesInRange() {
    const cameraX = camera.position.x / worldProperties.chunk_size;
    const cameraY = camera.position.y / worldProperties.chunk_size;
      for (let i = Math.round(cameraY - chunkViewRange); i < Math.round(cameraY + chunkViewRange); i++) {
        for (let j = Math.round(cameraX - chunkViewRange); j < Math.round(cameraX + chunkViewRange); j++) {
            if (j < 0 || i < 0 || j > worldProperties.world_width || i > worldProperties.world_height) {
                continue
            }
            updateTiles(j,i)
      } 
  }
}
async function updateEntities(x,y) {
    
  const chunkEntities = await getChunkEntities(x, y)
    Object.entries(chunkEntities).forEach(([key, value]) => {
	addEntity({key: value})
    });
}
async function updateTiles(x,y) {
    
  const chunkTiles = await getChunkTiles(x, y)
    for (let i = 0; i < worldProperties.chunk_size; i++) {
	for (let j = 0; j < worldProperties.chunk_size; j++) {
	    const texture = tileTextures[chunkTiles[j][i].tile_type]
	    addTile(chunkTiles[j][i],texture)
	}
    }
}
async function updateEntitiesInRange() {
    const cameraX = camera.position.x / worldProperties.chunk_size;
    const cameraY = camera.position.y / worldProperties.chunk_size;
      for (let i = Math.round(cameraY - chunkViewRange); i < Math.round(cameraY + chunkViewRange); i++) {
        for (let j = Math.round(cameraX - chunkViewRange); j < Math.round(cameraX + chunkViewRange); j++) {
            if (j < 0 || i < 0 || j > worldProperties.world_width || i > worldProperties.world_height) {
                continue
            }
            updateEntities(j,i)
      } 
  }
}
tick()
export async function createScene (el) {
  renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el })
  renderer.setSize(window.innerWidth, window.innerHeight)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  controls = new PointerLockControls(camera, renderer.domElement)
  renderer.shadowMap.enabled = true
  renderer.shadowMap.type = THREE.PCFSoftShadowMap

  await initScene()
  document.addEventListener('keydown', onKeyDown, false)
  worldProperties = await getWorldProperties()
  resize()
}

window.addEventListener('resize', resize)
