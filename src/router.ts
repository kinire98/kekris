import { createMemoryHistory, createRouter } from 'vue-router'
import Index from './pages/Index.vue'
import Board from "./pages/Board.vue";
import Main from "./pages/Main.vue";
import Multiplayer from './pages/multiplayer/Multiplayer.vue';
import Singleplayer from './pages/Singleplayer.vue';
import Settings from './pages/Settings.vue';
import ProfilePage from './pages/ProfilePage.vue';
import Controls from './pages/Controls.vue';
import Appeareance from './pages/Appeareance.vue';
import Language from './pages/Language.vue';
import Stats from './pages/Stats.vue';
import Again from './pages/Again.vue';
import Internet from './pages/multiplayer/Internet.vue';
import Local from './pages/multiplayer/Local.vue';
import Room from './pages/multiplayer/Room.vue';
import MultiplayerBoard from './pages/multiplayer/MultiplayerBoard.vue';

const routes = [
    { path: '/', component: Index },
    { path: '/main', component: Main },
    { path: '/multiplayer', component: Multiplayer },
    { path: '/singleplayer', component: Singleplayer },
    { path: '/settings', component: Settings },
    { path: '/settings/controls', component: Controls },
    { path: '/settings/appeareance', component: Appeareance },
    { path: '/settings/language', component: Language },
    { path: '/profile', component: ProfilePage },
    { path: '/classic', component: Board },
    { path: '/lines', component: Board },
    { path: '/blitz', component: Board },
    { path: '/stats', component: Stats },
    { path: '/again', component: Again },
    { path: '/internet', component: Internet },
    { path: '/local', component: Local },
    { path: '/host', component: Room },
    { path: '/join', component: Room },
    { path: '/rehost', component: Room },
    { path: '/rejoin', component: Room },
    { path: '/mutliplayer-board/:players/:id', component: MultiplayerBoard },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

