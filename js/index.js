
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
ctx.fillStyle = 'green';
ctx.fillRect(10, 10, 150, 100);

let js_state = {
    "key_down_queue":[],
}
window.addEventListener('keydown', function(event) {
    console.log("down");
    console.log(event.key);
    js_state.key_down_queue.push(event.key);
});

function int_to_rgb_str(color){
    let c_str = color.toString(16);
    length=c_str.length;
    for(let i=0;i<6-length;i++){
        c_str = "0"+c_str;
    }
    return "#"+c_str;
}
function draw(color,x,y,width,height){
    ctx.fillStyle = int_to_rgb_str(color);
    ctx.fillRect(x, y, width, height);
}
function clear(){
    ctx.clearRect(0, 0, 300, 300);
}
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
const frame_time_ms = (1.0/60.0)*1000.0;
const move_speed = 1;
console.log(int_to_rgb_str(0))
function game_loop(rust_module,state){
    clear();
    if(js_state.key_down_queue.length>=1){
        console.log(js_state.key_down_queue);
    }
    let x = 0;
    let y =0;
    for(let i in js_state.key_down_queue){
        if(js_state.key_down_queue[i]=="w"){
            y+=move_speed;
        }
        if(js_state.key_down_queue[i]=="s"){
            y-=move_speed;
        }
        if(js_state.key_down_queue[i]=="d"){
            x+=move_speed;
        }
        if(js_state.key_down_queue[i]=="a"){
            x-=move_speed;
        }
    }
    let v = {"x":x,"y":y};
    if(v.x!==0 && v.y !== 0){
        console.log(v)
    }
    let draw_calls = state.game_loop_js(v);
    if(draw_calls.length%5!=0){
        alert("Invalid Length")
    }
    for(let i=0;i<draw_calls.length;i+=5){
        draw(draw_calls[i],draw_calls[i+1],draw_calls[i+2],draw_calls[i+3],draw_calls[i+4])
    }
    //draw(0xff0000,x,y,10,10);
    js_state.key_down_queue=[]
    return state;
    
}
async function main(){
    module = await import("../pkg/index.js");
    let state = module.init_state_js();
    console.log(state)
    while(0==0){
        state = game_loop(module,state);
        await sleep(frame_time_ms)
    }
}
main();