import * as THREE from 'three'
import { PointerLockControls } from './PointerLockControls'
import axios from 'axios'
const scene = new THREE.Scene()
const material = new THREE.MeshBasicMaterial({
  color: 0xffffff
})
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
    unknown: 0x000000,
}
let worldProperties = null
const tiles = null
async function getChunk (x, y) {
  const response = await axios.get(`http://localhost:8081/tiles/${x}/${y}`)
  return response.data.tiles
}
async function getWorldProperties () {
  const response = await axios.get('http://localhost:8081/world_properties')
  return response.data
}
let renderer = null
let controls = null

const camera = new THREE.PerspectiveCamera(

  75,
  window.innerWidth / window.innerHeight,
  0.01,
  250

)
camera.up = new THREE.Vector3(0,0,1);
camera.rotation.y = 0// 3.14/4;
camera.rotation.x = 3.14/4
camera.position.x = 1.0
camera.position.y = 1.0
camera.position.z = 4.0

const tick = () => {
  if (renderer) {
    renderer.render(scene, camera)
  }
  if (controls) {
  }

  window.requestAnimationFrame(tick)
}

const resize = () => {
  renderer.setSize(window.innerWidth, window.innerHeight)
  camera.aspect = window.innerWidth / window.innerHeight
  camera.updateProjectionMatrix()
}

const onKeyDown = function (event) {
  switch (event.code) {
    case 'KeyW':
      controls.moveForward(0.25)
      break
    case 'KeyA':
      controls.moveRight(-0.25)
      break
    case 'KeyS':
      controls.moveForward(-0.25)
      break
    case 'KeyD':
      controls.moveRight(0.25)
      break
  }
}
function addTile (tile, surroundingTiles) {
  const geometry = new THREE.BoxGeometry(1,1,tile.h/200)


  let color = 0xff0000 
    if (tile.tile_type in tileColors) {
        color = tileColors[tile.tile_type]
    }
    else {
        color = tileColors["unknown"]
    }
  const material = new THREE.MeshBasicMaterial({ color: color})
  const mesh = new THREE.Mesh(geometry, material)
  mesh.position.set(tile.x,tile.y,0);
  scene.add(mesh)
}
async function initScene () {
  const chunkTiles = await getChunk(0, 0)
  worldProperties = await getWorldProperties()
  for (let i = 0; i < worldProperties.chunk_size; i++) {
    for (let j = 0; j < worldProperties.chunk_size; j++) {
        const tile = chunkTiles[j][i]
        const surroundingTiles = []
        surroundingTiles[0] = chunkTiles?.[j-1]?.[i-1] ?? {x:0,y:0,height:0}
        surroundingTiles[1] = chunkTiles?.[j+1]?.[i-1] ?? {x:0,y:0,height:0}
        surroundingTiles[2] = chunkTiles?.[j-1]?.[i+1] ?? {x:0,y:0,height:0}
        surroundingTiles[3] = chunkTiles?.[j+1]?.[i+1] ?? {x:0,y:0,height:0}
        addTile(tile, surroundingTiles)
    }
  }
}
tick()
export async function createScene (el) {
  renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el })
  renderer.setSize(window.innerWidth, window.innerHeight)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  controls = new PointerLockControls(camera, renderer.domElement)
  await initScene()
  document.addEventListener('keydown', onKeyDown, false)
  worldProperties = await getWorldProperties()
  resize()
}

window.addEventListener('resize', resize)
