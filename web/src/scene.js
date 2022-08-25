import * as THREE from 'three'
import { PointerLockControls } from './PointerLockControls'
import axios from 'axios'
import Renderer from './render'
import Update from './update'
import Network from './network'
const scene = new THREE.Scene()

let worldProperties = null
const tiles = null
const entities = null
let renderer = null;
let network = null;
let update = null;
let controls = null
const clearEntityTime = 1000;
let clearEntityChange = 0;
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
  renderer && renderer.renderScene();
  if (controls) {
  }
  light.target.updateMatrixWorld()
  
  update && update.refreshEntities()
  window.requestAnimationFrame(tick)

  }, 10)}

const resize = () => {
  renderer.rend.setSize(window.innerWidth, window.innerHeight)
  camera.aspect = window.innerWidth / window.innerHeight
  camera.updateProjectionMatrix()
}

const onKeyDown = function (event) {
  switch (event.code) {
    case 'KeyW':
      update.refreshChunk(true)
      controls.moveForward(1)
      break
    case 'KeyA':
      update.refreshChunk(false)
      controls.moveRight(-1)
      break
    case 'KeyS':
      update.refreshChunk(true)
      controls.moveForward(-1)
      break
    case 'KeyD':
      update.refreshChunk(false)
      controls.moveRight(1)
      break
  }
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
  worldProperties = await network.getWorldProperties()

  update.updateTilesInRange()
}
tick()
export async function createScene (el) {
  document.getElementById("mainCanvas").style.display = "none"
    document.getElementById("playButton").onclick = async () => {
	
	network = new Network()
	worldProperties = await network.getWorldProperties()
	update = new Update(scene, camera,worldProperties)
	renderer = new Renderer(el, camera, scene, worldProperties)
	
    const x = Math.random() * (worldProperties.chunk_size * worldProperties.world_width - 0) + 0
    const y = Math.random() * (worldProperties.chunk_size * worldProperties.world_height - 0) + 0
    const name = "adsf"
    const id = 10
    await network.addPlayer(worldProperties, x, y, name, id )
    controls = new PointerLockControls(camera, renderer.rend.domElement)
    await initScene()
	document.addEventListener('keydown', onKeyDown, false)
	resize()
	document.getElementById("mainCanvas").style.display = "block"
	document.getElementById("loginScreen").style.display = "none"
    }
    
}

window.addEventListener('resize', resize)
