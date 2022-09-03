window.addEventListener("DOMContentLoaded", () => main());
const { listen } = window.__TAURI__.event;
const { invoke } = window.__TAURI__.tauri;

/** @type { HTMLDivElement } */
let channel;
/** @type { HTMLDivElement } */
let channel_info;
/** @type { HTMLDivElement } */
let message_container;

async function main () {
  channel = document.getElementById('channels');
  channel_info = document.getElementById('channel-info'); 
  message_container = document.getElementById('messages');
}

listen('add_channel', ({ payload }) => {
  const { id, name } = payload;
  const node = channel_node(name, id);
  channel.appendChild(node);
});

listen('set_current_channel', ({ payload }) => {
  console.debug(payload);
  const { channel: { id, name }, messages } = payload;
  channel_info.innerText = `${name} # ${id}`;
  message_container.innerHTML = "";
    for(const { content, id } of messages) {
      const node = message_node(content, id);
      message_container.appendChild(node);
  }
});

function message_node(content, id) {
  let result = document.createElement("div");
  result.className = "message";
  result.id = id;
  result.innerHTML = content;
  return result;
}

function channel_node(name, id) {
  let result = document.createElement("div");
  result.className = "channel";
  result.id = id;
  result.innerHTML = `# ${name}`;
  result.addEventListener("click", () => {
    invoke('channel_select', { id: id});
  })

  return result;
}

////


async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}
