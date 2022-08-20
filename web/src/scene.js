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
    ash: 0x525245,
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
camera.rotation.y = 0// 3.14/4;
camera.rotation.x = 0
camera.position.x = 0
camera.position.y = 0.56
camera.position.z = 1.1

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
function addTile (tile) {
  const geometry = new THREE.BufferGeometry()
  const vertices = new Float32Array([
    tile.x, tile.y, 0.0,
	 tile.x + 1.0, tile.y + 0.0, 0.0,
	 tile.x + 1.0, tile.y + 1.0, 0.0,

	 tile.x + 1.0, tile.y + 1.0, 0.0,
    tile.x + 0.0, tile.y + 1.0, 0.0,
    tile.x + 0.0, tile.y + 0.0, 0.0
  ])

  geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3))
  let color = 0xff0000 
    if (tile.tile_type in tileColors) {
        color = tileColors[tile.tile_type]
    }
    else {
        color = tileColors["unknown"]
    }
    console.log(tile.tile_type)
  const material = new THREE.MeshBasicMaterial({ color: color})
  const mesh = new THREE.Mesh(geometry, material)
  scene.add(mesh)
}
async function initScene () {
  const chunkTiles = await getChunk(0, 0)
  worldProperties = await getWorldProperties()
  for (let i = 0; i < worldProperties.chunk_size; i++) {
    for (let j = 0; j < worldProperties.chunk_size; j++) {
        const tile = chunkTiles[j][i]
        addTile(tile)
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
