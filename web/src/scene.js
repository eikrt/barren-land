import * as THREE from 'three'
import { PointerLockControls } from './PointerLockControls'
import axios from 'axios'
const scene = new THREE.Scene()
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

const loader = new THREE.TextureLoader();
let worldProperties = null
const tiles = null
let renderer = null
let controls = null

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
),
}
async function getChunk (x, y) {
  const response = await axios.get(`http://localhost:8081/tiles/${x}/${y}`)
  return response.data.tiles
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
camera.up = new THREE.Vector3(0,0,1);
camera.rotation.y = 0// 3.14/4;
camera.rotation.x = 3.14/4
camera.position.x = 1.0
camera.position.y = 1.0
camera.position.z = 4.0
const light = new THREE.DirectionalLight( 0xffffff, 1 );

const tick = () => {
  if (renderer) {
    renderer.render(scene, camera)
  }
  if (controls) {
  }
    light.target.updateMatrixWorld();

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
function addTile (tile, cubeTexture) {
  const geometry = new THREE.BoxGeometry(1,1,tile.h/200)


  let color = 0xff0000 
    if (tile.tile_type in tileColors) {
        color = tileColors[tile.tile_type]
    }
    else {
        color = tileColors["unknown"]
    }
  const material = new THREE.MeshStandardMaterial({ color: color, map: cubeTexture})
  const mesh = new THREE.Mesh(geometry, material)
  mesh.position.set(tile.x,tile.y,0);
mesh.castShadow = true; //default is false
mesh.receiveShadow = true ; //default
  scene.add(mesh)
}
async function initScene () {
    
light.position.set( 12, 10, 32 ); //default; light shining from top
light.castShadow = true; // default false
scene.add( light );
scene.add( light.target );

light.shadow.mapSize.width = 512; // default
light.shadow.mapSize.height = 512; // default
light.shadow.camera.near = 0.5; // default
light.shadow.camera.far = 1000; // default
    light.intensity = 1;
  const chunkTiles = await getChunk(0, 0)
  worldProperties = await getWorldProperties()
  for (let i = 0; i < worldProperties.chunk_size; i++) {
    for (let j = 0; j < worldProperties.chunk_size; j++) {
        const tile = chunkTiles[j][i]
        let texture = tileTextures[tile.tile_type];
        addTile(tile, texture)
    }
  }
}
tick()
export async function createScene (el) {
  renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el })
  renderer.setSize(window.innerWidth, window.innerHeight)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  controls = new PointerLockControls(camera, renderer.domElement)
renderer.shadowMap.enabled = true;
renderer.shadowMap.type = THREE.PCFSoftShadowMap;

  await initScene()
  document.addEventListener('keydown', onKeyDown, false)
  worldProperties = await getWorldProperties()
  resize()
}

window.addEventListener('resize', resize)
