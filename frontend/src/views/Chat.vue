<template>
  <div class="h-screen flex bg-gradient-to-br from-pink-50 to-pink-100">
    <!-- 侧边栏 -->
    <aside class="w-80 bg-white/80 backdrop-blur-sm rounded-r-3xl shadow-lg p-4">
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
          <div class="w-12 h-12 rounded-full bg-gradient-to-br from-pink-400 to-pink-600 flex items-center justify-center text-white font-bold">
            {{ userInitial }}
          </div>
          <div>
            <h3 class="font-bold text-gray-700">{{ user?.username || '用户' }}</h3>
            <p class="text-xs text-green-500">在线</p>
          </div>
        </div>
        <button 
          @click="logout"
          class="text-gray-500 hover:text-red-500 transition-colors"
          title="退出登录"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
          </svg>
        </button>
      </div>
      
      <div class="mb-4 relative">
        <div class="flex gap-2 mb-2">
          <button 
            @click="searchType = 'username'"
            :class="searchType === 'username' ? 'bg-pink-500 text-white' : 'bg-pink-50 text-pink-600'"
            class="flex-1 py-1 rounded-full text-xs font-medium transition-colors"
          >
            用户名搜索
          </button>
          <button 
            @click="searchType = 'email'"
            :class="searchType === 'email' ? 'bg-pink-500 text-white' : 'bg-pink-50 text-pink-600'"
            class="flex-1 py-1 rounded-full text-xs font-medium transition-colors"
          >
            邮箱搜索
          </button>
        </div>
        <input 
          type="text" 
          v-model="searchQuery"
          @input="searchUsers"
          :placeholder="searchType === 'email' ? '输入邮箱地址搜索...' : '搜索用户添加好友...'" 
          class="w-full px-4 py-2 rounded-full border-2 border-pink-100 focus:border-pink-300 focus:outline-none transition-colors"
        />
        <div v-if="searchResults.length > 0" class="absolute z-10 w-full mt-1 bg-white rounded-2xl shadow-lg border border-pink-100 max-h-60 overflow-y-auto">
          <div 
            v-for="u in searchResults" 
            :key="u.id"
            class="flex items-center justify-between p-3 hover:bg-pink-50 cursor-pointer transition-colors"
          >
            <div class="flex items-center gap-3">
              <div class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold text-sm">
                {{ u.username.charAt(0).toUpperCase() }}
              </div>
              <div>
                <p class="font-medium text-gray-700 text-sm">{{ u.username }}</p>
                <p v-if="searchType === 'email'" class="text-xs text-gray-500">{{ u.email }}</p>
                <p v-else class="text-xs text-gray-500">{{ u.status === 'online' ? '在线' : '离线' }}</p>
              </div>
            </div>
            <button 
              @click.stop="addFriend(u.id)"
              class="px-3 py-1 rounded-full bg-pink-500 text-white text-xs font-medium hover:bg-pink-600 transition-colors"
            >
              添加
            </button>
          </div>
        </div>
        <div v-if="searchQuery && searchType === 'email' && searchQuery.includes('@') && !searchResults.length && !isSearching" 
             class="absolute z-10 w-full mt-1 bg-white rounded-2xl shadow-lg border border-pink-100 p-4 text-center">
          <p class="text-sm text-gray-500 mb-2">未找到该邮箱用户</p>
          <button 
            @click="addFriendByEmail"
            class="px-4 py-2 rounded-full bg-pink-500 text-white text-sm font-medium hover:bg-pink-600 transition-colors"
          >
            尝试添加为好友
          </button>
        </div>
      </div>
      
      <div class="flex gap-2 mb-4">
        <button 
          @click="activeTab = 'friends'"
          :class="activeTab === 'friends' ? 'bg-pink-500 text-white' : 'bg-pink-100 text-pink-600 hover:bg-pink-200'"
          class="flex-1 py-2 rounded-full font-medium transition-colors"
        >
          好友
        </button>
        <button 
          @click="activeTab = 'groups'"
          :class="activeTab === 'groups' ? 'bg-pink-500 text-white' : 'bg-pink-100 text-pink-600 hover:bg-pink-200'"
          class="flex-1 py-2 rounded-full font-medium transition-colors"
        >
          群聊
        </button>
      </div>
      
      <!-- 好友列表 -->
      <div v-if="activeTab === 'friends'" class="space-y-2">
        <div 
          v-for="u in users" 
          :key="u.id"
          @click="selectUser(u, 'private')"
          :class="selectedContact?.id === u.id ? 'p-3 rounded-2xl bg-pink-50 cursor-pointer transition-colors' : 'p-3 rounded-2xl hover:bg-pink-50 cursor-pointer transition-colors'"
        >
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold">
              {{ u.username.charAt(0).toUpperCase() }}
            </div>
            <div class="flex-1">
              <p class="font-medium text-gray-700">{{ u.username }}</p>
              <p class="text-xs text-gray-500 truncate">{{ u.status === 'online' ? '在线' : '离线' }}</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 群聊列表 -->
      <div v-else class="space-y-2">
        <div class="mb-3">
          <button 
            @click="showCreateGroup = true"
            class="w-full py-2 rounded-2xl border-2 border-dashed border-pink-300 text-pink-500 font-medium hover:bg-pink-50 transition-colors"
          >
            + 创建群聊
          </button>
        </div>
        <div 
          v-for="g in groups" 
          :key="g.id"
          @click="selectGroup(g)"
          :class="selectedGroup?.id === g.id ? 'p-3 rounded-2xl bg-pink-50 cursor-pointer transition-colors' : 'p-3 rounded-2xl hover:bg-pink-50 cursor-pointer transition-colors'"
        >
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-full bg-gradient-to-br from-purple-400 to-purple-600 flex items-center justify-center text-white font-bold">
              {{ g.name.charAt(0) }}
            </div>
            <div class="flex-1">
              <p class="font-medium text-gray-700">{{ g.name }}</p>
              <p class="text-xs text-gray-500">群聊</p>
            </div>
          </div>
        </div>
      </div>
    </aside>

    <!-- 聊天区域 -->
    <main class="flex-1 flex flex-col mx-4">
      <!-- 聊天头部 -->
      <header class="bg-white/80 backdrop-blur-sm rounded-t-3xl p-4 shadow-sm">
        <div v-if="chatType === 'private' && selectedContact" class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold">
            {{ selectedUserInitial }}
          </div>
          <div>
            <h3 class="font-bold text-gray-700">{{ selectedContact.username }}</h3>
            <p class="text-xs text-green-500">{{ selectedContact.status === 'online' ? '在线' : '离线' }}</p>
          </div>
        </div>
        <div v-else-if="chatType === 'group' && selectedGroup" class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-full bg-gradient-to-br from-purple-400 to-purple-600 flex items-center justify-center text-white font-bold">
            {{ selectedGroup.name.charAt(0) }}
          </div>
          <div>
            <h3 class="font-bold text-gray-700">{{ selectedGroup.name }}</h3>
            <p class="text-xs text-gray-500">{{ groupMembers.length }} 位成员</p>
          </div>
        </div>
        <div v-else class="text-center text-gray-500">
          请选择一个聊天开始
        </div>
      </header>
      
      <!-- 消息列表 -->
      <div class="flex-1 bg-white/60 backdrop-blur-sm overflow-y-auto p-4 space-y-4" ref="messagesContainer">
        <div v-if="!selectedContact && !selectedGroup" class="text-center text-gray-500 mt-10">
          请选择一个聊天开始
        </div>
        <div v-else-if="messages.length === 0" class="text-center text-gray-500 mt-10">
          暂无消息，开始聊天吧！
        </div>
        <div v-else v-for="message in messages" :key="message.id" :class="message.sender_id === currentUserId ? 'flex gap-3 justify-end' : 'flex gap-3'">
          <template v-if="message.sender_id !== currentUserId">
            <div class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold text-sm flex-shrink-0">
              {{ getSenderInitial(message.sender_id) }}
            </div>
            <div class="max-w-md">
              <p v-if="chatType === 'group'" class="text-xs text-gray-500 mb-1">{{ getSenderName(message.sender_id) }}</p>
              <div class="bg-white rounded-2xl rounded-tl-none px-4 py-2 shadow-sm">
                <p class="text-gray-700">{{ message.content }}</p>
              </div>
              <span class="text-xs text-gray-400 mt-1 block">{{ formatTime(message.timestamp) }}</span>
            </div>
          </template>
          <template v-else>
            <div class="max-w-md">
              <div class="bg-gradient-to-br from-pink-500 to-pink-600 rounded-2xl rounded-tr-none px-4 py-2 shadow-sm">
                <p class="text-white">{{ message.content }}</p>
              </div>
              <span class="text-xs text-gray-400 mt-1 block text-right">{{ formatTime(message.timestamp) }}</span>
            </div>
          </template>
        </div>
      </div>
      
      <!-- 输入区域 -->
      <div class="bg-white/80 backdrop-blur-sm rounded-b-3xl p-4 shadow-sm">
        <div v-if="showEmojiPicker" class="absolute bottom-20 left-8 bg-white rounded-2xl shadow-lg p-4 border border-pink-100 z-50 w-64">
          <div class="flex flex-wrap gap-2">
            <button 
              v-for="emoji in emojiList" 
              :key="emoji"
              @click="addEmoji(emoji)"
              class="text-2xl w-10 h-10 flex items-center justify-center hover:bg-pink-50 rounded-full transition-colors"
            >
              {{ emoji }}
            </button>
          </div>
        </div>
        
        <div class="flex gap-3">
          <button 
            @click="showEmojiPicker = !showEmojiPicker"
            class="w-10 h-10 rounded-full bg-pink-100 text-pink-500 flex items-center justify-center hover:bg-pink-200 transition-colors relative"
          >
            😊
          </button>
          <button class="w-10 h-10 rounded-full bg-pink-100 text-pink-500 flex items-center justify-center hover:bg-pink-200 transition-colors">
            📎
          </button>
          <input 
            type="text" 
            v-model="newMessage"
            @keyup.enter="sendMessage"
            placeholder="输入消息..." 
            class="flex-1 px-4 py-2 rounded-full border-2 border-pink-100 focus:border-pink-300 focus:outline-none transition-colors"
          />
          <button 
            @click="sendMessage"
            class="px-6 py-2 rounded-full bg-gradient-to-r from-pink-500 to-pink-600 text-white font-medium hover:from-pink-600 hover:to-pink-700 transition-all hover:shadow-lg"
          >
            发送
          </button>
        </div>
      </div>
    </main>

    <!-- 右侧面板 -->
    <aside class="w-72 bg-white/80 backdrop-blur-sm rounded-l-3xl shadow-lg p-4">
      <!-- 私聊时显示好友信息 -->
      <div v-if="chatType === 'private' && selectedContact">
        <h3 class="font-bold text-gray-700 mb-4">好友信息</h3>
        <div class="flex flex-col items-center text-center">
          <div class="w-20 h-20 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold text-2xl mb-3">
            {{ selectedUserInitial }}
          </div>
          <p class="font-bold text-gray-700 text-lg">{{ selectedContact.username }}</p>
          <p class="text-sm" :class="selectedContact.status === 'online' ? 'text-green-500' : 'text-gray-400'">
            {{ selectedContact.status === 'online' ? '在线' : '离线' }}
          </p>
          <div class="mt-6 w-full space-y-3">
            <div class="bg-pink-50 rounded-xl p-3">
              <p class="text-xs text-gray-500">用户ID</p>
              <p class="text-sm text-gray-700 font-mono">{{ selectedContact.id }}</p>
            </div>
            <div class="w-full space-y-2">
              <button 
                @click="clearChatHistory"
                class="w-full py-2 rounded-xl bg-pink-100 text-pink-600 font-medium hover:bg-pink-200 transition-colors"
              >
                清空聊天记录
              </button>
              <button 
                @click="removeFriend"
                class="w-full py-2 rounded-xl bg-red-100 text-red-600 font-medium hover:bg-red-200 transition-colors"
              >
                删除好友
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 群聊时显示群成员 -->
      <div v-else-if="chatType === 'group' && selectedGroup">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-bold text-gray-700">群成员 ({{ groupMembers.length }})</h3>
          <button 
            @click="showAddMember = true"
            class="text-sm text-pink-500 hover:text-pink-600"
          >
            + 添加
          </button>
        </div>
        <div class="space-y-3">
          <div 
            v-for="member in groupMembers" 
            :key="member.id"
            class="flex items-center gap-3 p-2 rounded-xl hover:bg-pink-50 cursor-pointer transition-colors"
          >
            <div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold">
              {{ member.username.charAt(0).toUpperCase() }}
            </div>
            <div>
              <p class="font-medium text-gray-700">{{ member.username }}</p>
              <p class="text-xs text-green-500">{{ member.status === 'online' ? '在线' : '离线' }}</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 未选择聊天时 -->
      <div v-else class="text-center text-gray-500 mt-10">
        选择聊天查看详情
      </div>
    </aside>

    <!-- 创建群聊弹窗 -->
    <div v-if="showCreateGroup" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-3xl p-6 w-96 shadow-2xl">
        <h3 class="font-bold text-gray-700 text-lg mb-4">创建群聊</h3>
        <input 
          v-model="newGroupName"
          type="text" 
          placeholder="群聊名称" 
          class="w-full px-4 py-2 rounded-full border-2 border-pink-100 focus:border-pink-300 focus:outline-none transition-colors mb-4"
        />
        <p class="text-sm text-gray-500 mb-2">选择好友加入：</p>
        <div class="max-h-48 overflow-y-auto space-y-2 mb-4">
          <label 
            v-for="u in users" 
            :key="u.id"
            class="flex items-center gap-3 p-2 rounded-xl hover:bg-pink-50 cursor-pointer"
          >
            <input 
              type="checkbox" 
              :value="u.id"
              v-model="selectedMemberIds"
              class="w-4 h-4 text-pink-500 rounded focus:ring-pink-300"
            />
            <div class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold text-sm">
              {{ u.username.charAt(0).toUpperCase() }}
            </div>
            <span class="text-gray-700">{{ u.username }}</span>
          </label>
        </div>
        <div class="flex gap-3">
          <button 
            @click="showCreateGroup = false"
            class="flex-1 py-2 rounded-full bg-gray-100 text-gray-600 font-medium hover:bg-gray-200 transition-colors"
          >
            取消
          </button>
          <button 
            @click="createGroup"
            class="flex-1 py-2 rounded-full bg-gradient-to-r from-pink-500 to-pink-600 text-white font-medium hover:from-pink-600 hover:to-pink-700 transition-all"
          >
            创建
          </button>
        </div>
      </div>
    </div>

    <!-- 添加群成员弹窗 -->
    <div v-if="showAddMember" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-3xl p-6 w-96 shadow-2xl">
        <h3 class="font-bold text-gray-700 text-lg mb-4">添加群成员</h3>
        <div class="max-h-48 overflow-y-auto space-y-2 mb-4">
          <label 
            v-for="u in users.filter(u => !groupMembers.some(m => m.id === u.id))" 
            :key="u.id"
            class="flex items-center gap-3 p-2 rounded-xl hover:bg-pink-50 cursor-pointer"
          >
            <input 
              type="checkbox" 
              :value="u.id"
              v-model="addMemberIds"
              class="w-4 h-4 text-pink-500 rounded focus:ring-pink-300"
            />
            <div class="w-8 h-8 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold text-sm">
              {{ u.username.charAt(0).toUpperCase() }}
            </div>
            <span class="text-gray-700">{{ u.username }}</span>
          </label>
        </div>
        <div class="flex gap-3">
          <button 
            @click="showAddMember = false"
            class="flex-1 py-2 rounded-full bg-gray-100 text-gray-600 font-medium hover:bg-gray-200 transition-colors"
          >
            取消
          </button>
          <button 
            @click="addMembersToGroup"
            class="flex-1 py-2 rounded-full bg-gradient-to-r from-pink-500 to-pink-600 text-white font-medium hover:from-pink-600 hover:to-pink-700 transition-all"
          >
            添加
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed } from 'vue'
import { useRouter } from 'vue-router'
import api from '../services/api'

const router = useRouter()

let ws: WebSocket | null = null

interface Message {
  id: string
  sender_id: string
  receiver_id: string
  content: string
  timestamp: number
  is_read: boolean
}

interface User {
  id: string
  username: string
  email: string
  avatar_url: string | null
  status: string
}

interface Contact {
  id: string
  username: string
  status: string
}

interface Group {
  id: string
  name: string
  creator_id: string
  avatar_url: string | null
  created_at: number
}

interface GroupMember {
  id: string
  username: string
  status: string
}

const user = ref<User | null>(null)
const users = ref<Contact[]>([])
const selectedContact = ref<Contact | null>(null)
const selectedGroup = ref<Group | null>(null)
const groupMembers = ref<GroupMember[]>([])
const messages = ref<Message[]>([])
const groups = ref<Group[]>([])

const newMessage = ref('')
const currentUserId = ref('')
const messagesContainer = ref<HTMLElement | null>(null)
const searchQuery = ref('')
const searchResults = ref<User[]>([])
const searchType = ref<'username' | 'email'>('username')
const isSearching = ref(false)
const showEmojiPicker = ref(false)
const activeTab = ref<'friends' | 'groups'>('friends')
const chatType = ref<'private' | 'group'>('private')
const showCreateGroup = ref(false)
const showAddMember = ref(false)
const newGroupName = ref('')
const selectedMemberIds = ref<string[]>([])
const addMemberIds = ref<string[]>([])

const emojiList = ref([
  '😊', '😂', '😍', '🤔', '😢', '😡', '👍', '👎',
  '❤️', '🎉', '🔥', '🥳', '🤩', '😎', '🤗', '🙏',
  '😴', '🤣', '😅', '😆', '😋', '😌', '😍', '🤩',
  '😎', '🤓', '😡', '😢', '😭', '😱', '😨', '😴'
])

const userInitial = computed(() => {
  return user.value?.username?.charAt(0).toUpperCase() || 'U'
})

const selectedUserInitial = computed(() => {
  return selectedContact.value?.username?.charAt(0).toUpperCase() || 'A'
})

const selectUser = (contact: Contact, type: 'private' | 'group' = 'private') => {
  chatType.value = type
  selectedContact.value = contact
  selectedGroup.value = null
  messages.value = []
  loadMessages(contact.id)
}

const selectGroup = async (group: Group) => {
  chatType.value = 'group'
  selectedGroup.value = group
  selectedContact.value = null
  messages.value = []
  
  await loadGroupMembers(group.id)
  loadMessages(group.id)
  loadFriends() // 加载好友列表，而不是所有用户
}

const loadMessages = async (targetId: string) => {
  try {
    const params = chatType.value === 'group' 
      ? { group_id: targetId }
      : { contact_id: targetId }
    
    const response = await api.get('/messages', { params })
    messages.value = response.data
    scrollToBottom()
  } catch (error) {
    console.error('Failed to load messages:', error)
    messages.value = []
  }
}

const loadGroupMembers = async (groupId: string) => {
  try {
    const response = await api.get(`/groups/${groupId}/members`)
    groupMembers.value = response.data
  } catch (error) {
    console.error('Failed to load group members:', error)
    groupMembers.value = []
  }
}

const getSenderInitial = (senderId: string) => {
  if (senderId === currentUserId.value) return userInitial.value
  if (chatType.value === 'private' && selectedContact.value?.id === senderId) {
    return selectedContact.value.username.charAt(0).toUpperCase()
  }
  const member = groupMembers.value.find(m => m.id === senderId)
  return member?.username?.charAt(0).toUpperCase() || '?'
}

const getSenderName = (senderId: string) => {
  if (senderId === currentUserId.value) return user.value?.username || '我'
  if (chatType.value === 'private' && selectedContact.value?.id === senderId) {
    return selectedContact.value.username
  }
  const member = groupMembers.value.find(m => m.id === senderId)
  return member?.username || '未知'
}

let searchTimeout: number | null = null

const searchUsers = async () => {
  if (searchTimeout) clearTimeout(searchTimeout)
  isSearching.value = true
  searchTimeout = setTimeout(async () => {
    if (!searchQuery.value.trim()) {
      searchResults.value = []
      isSearching.value = false
      return
    }
    try {
      if (searchType.value === 'email') {
        const response = await api.get('/users/search-by-email', {
          params: { email: searchQuery.value }
        })
        searchResults.value = response.data
      } else {
        const response = await api.get('/users/search', {
          params: { username: searchQuery.value }
        })
        searchResults.value = response.data
      }
    } catch (error) {
      console.error('Failed to search users:', error)
      searchResults.value = []
    } finally {
      isSearching.value = false
    }
  }, 300)
}

const addFriend = async (friendId: string) => {
  try {
    await api.post('/friends/add', { friend_id: friendId })
    searchResults.value = searchResults.value.filter(u => u.id !== friendId)
    searchQuery.value = ''
    await loadFriends()
  } catch (error: any) {
    const msg = error.response?.data?.error || '添加好友失败'
    alert(msg)
  }
}

const addFriendByEmail = async () => {
  if (!searchQuery.value.trim()) return
  try {
    await api.post('/friends/add-by-email', { email: searchQuery.value })
    searchQuery.value = ''
    searchResults.value = []
    await loadFriends()
  } catch (error: any) {
    const msg = error.response?.data?.error || '添加好友失败'
    alert(msg)
  }
}

const loadFriends = async () => {
  try {
    const response = await api.get('/friends')
    users.value = response.data
    if (users.value.length > 0 && !selectedContact.value && activeTab.value === 'friends') {
      selectUser(users.value[0])
    }
  } catch (error) {
    console.error('Failed to load friends:', error)
  }
}

const loadAllUsers = async () => {
  try {
    const response = await api.get('/users')
    users.value = response.data
  } catch (error) {
    console.error('Failed to load users:', error)
  }
}

const loadGroups = async () => {
  try {
    const response = await api.get('/groups')
    groups.value = response.data
  } catch (error) {
    console.error('Failed to load groups:', error)
    groups.value = []
  }
}

const createGroup = async () => {
  if (!newGroupName.value.trim()) {
    alert('请输入群聊名称')
    return
  }
  if (selectedMemberIds.value.length === 0) {
    alert('请至少选择一位好友')
    return
  }
  
  try {
    const response = await api.post('/groups', {
      name: newGroupName.value,
      member_ids: selectedMemberIds.value
    })
    groups.value.push(response.data)
    showCreateGroup.value = false
    newGroupName.value = ''
    selectedMemberIds.value = []
    await loadGroups()
  } catch (error: any) {
    alert(error.response?.data?.error || '创建群聊失败')
  }
}

const addMembersToGroup = async () => {
  if (!selectedGroup.value || addMemberIds.value.length === 0) return
  
  try {
    for (const userId of addMemberIds.value) {
      await api.post(`/groups/${selectedGroup.value.id}/members`, { user_id: userId })
    }
    showAddMember.value = false
    addMemberIds.value = []
    await loadGroupMembers(selectedGroup.value.id)
  } catch (error: any) {
    alert(error.response?.data?.error || '添加成员失败')
  }
}

const logout = async () => {
  try {
    await api.put('/users/status', { status: 'offline' })
  } catch (error) {
    console.error('Failed to update status:', error)
  }
  
  localStorage.removeItem('token')
  localStorage.removeItem('user')
  router.push('/login')
}

const clearChatHistory = () => {
  if (confirm('确定要清空聊天记录吗？此操作不可恢复。')) {
    messages.value = []
  }
}

const removeFriend = async () => {
  if (!selectedContact.value) return
  
  if (confirm(`确定要删除好友 ${selectedContact.value.username} 吗？`)) {
    try {
      await api.delete('/friends/remove', {
        data: { friend_id: selectedContact.value.id }
      })
      
      // 重新加载好友列表
      await loadFriends()
      
      // 清空聊天记录
      messages.value = []
      
      // 取消选择当前好友
      selectedContact.value = null
    } catch (error: any) {
      const msg = error.response?.data?.error || '删除好友失败'
      alert(msg)
    }
  }
}

const formatTime = (timestamp: number) => {
  const date = new Date(timestamp)
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
}

const scrollToBottom = () => {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}

const addEmoji = (emoji: string) => {
  newMessage.value += emoji
  showEmojiPicker.value = false
}

const sendMessage = async () => {
  if (!newMessage.value.trim()) return
  
  let receiverId = ''
  if (chatType.value === 'private' && selectedContact.value) {
    receiverId = selectedContact.value.id
  } else if (chatType.value === 'group' && selectedGroup.value) {
    receiverId = selectedGroup.value.id
  }
  
  if (!receiverId) return
  
  try {
    const response = await api.post('/messages', {
      receiver_id: receiverId,
      content: newMessage.value,
      message_type: 'text'
    })
    // 对于私聊消息，直接添加到消息列表
    // 对于群聊消息，等待 WebSocket 通知添加，避免重复
    if (chatType.value === 'private') {
      messages.value.push(response.data)
      scrollToBottom()
    }
    newMessage.value = ''
  } catch (error) {
    console.error('Failed to send message:', error)
  }
}

const connectWebSocket = () => {
  const userStr = localStorage.getItem('user')
  if (!userStr) return
  
  const userData = JSON.parse(userStr)
  const wsUrl = `ws://localhost:8080/ws?user_id=${userData.id}`
  ws = new WebSocket(wsUrl)
  
  ws.onopen = () => {
    console.log('WebSocket connected')
  }
  
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data)
    if (data.type === 'status_update') {
      users.value = users.value.map(u => 
        u.id === data.user_id ? { ...u, status: data.status } : u
      )
      groupMembers.value = groupMembers.value.map(m =>
        m.id === data.user_id ? { ...m, status: data.status } : m
      )
    } else if (data.type === 'new_message') {
      const newMsg = data.data
      const exists = messages.value.some(m => m.id === newMsg.id)
      if (!exists) {
        if (chatType.value === 'private' && selectedContact.value && 
            (newMsg.sender_id === selectedContact.value.id || newMsg.receiver_id === selectedContact.value.id)) {
          messages.value.push(newMsg)
          scrollToBottom()
        } else if (chatType.value === 'group' && selectedGroup.value &&
                   newMsg.receiver_id === selectedGroup.value.id) {
          messages.value.push(newMsg)
          scrollToBottom()
        }
      }
    }
  }
  
  ws.onclose = () => {
    console.log('WebSocket disconnected')
    setTimeout(connectWebSocket, 3000)
  }
  
  ws.onerror = (error) => {
    console.error('WebSocket error:', error)
  }
}

onMounted(() => {
  const token = localStorage.getItem('token')
  if (!token) {
    router.push('/login')
    return
  }
  
  const userStr = localStorage.getItem('user')
  if (userStr) {
    user.value = JSON.parse(userStr)
    currentUserId.value = user.value.id
  }
  loadFriends()
  loadGroups()
  connectWebSocket()
})

onUnmounted(() => {
  if (ws) {
    ws.close()
  }
})
</script>

<style scoped>
</style>