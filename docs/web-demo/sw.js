const CACHE_NAME = "othello-v1";
const FILES_TO_CACHE = [
	"./",
	"./index.html",
	"./manifest.json",
	"./icon.png",
	"./mq_js_bundle.js",
	"./othello.wasm",
];

self.addEventListener("install", (event) => {
	event.waitUntil(caches.open(CACHE_NAME).then((cache) => cache.addAll(FILES_TO_CACHE)));
});

self.addEventListener("fetch", (event) => {
	event.respondWith(caches.match(event.request).then((cached) => cached || fetch(event.request)));
});
