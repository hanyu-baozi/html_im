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
            <p class="text-xs" :class="userStatus === 'online' ? 'text-green-500' : 'text-red-500'">{{ userStatus === 'online' ? '在线' : '离线' }}</p>
          </div>
        </div>
        <div class="flex gap-1">
          <button 
            v-if="isAdmin"
            @click="showAdminPanel = true; adminTab = 'messages'; conversationPage = 1; loadConversations()"
            class="text-gray-500 hover:text-blue-500 transition-colors"
            title="管理员面板"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </button>
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
            <div class="relative">
              <div class="w-10 h-10 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-bold">
                {{ u.username.charAt(0).toUpperCase() }}
              </div>
              <div v-if="unreadCounts[u.id] > 0" class="absolute -top-1 -right-1 w-5 h-5 bg-red-500 rounded-full flex items-center justify-center text-white text-xs font-bold border-2 border-white">
                {{ unreadCounts[u.id] > 99 ? '99+' : unreadCounts[u.id] }}
              </div>
            </div>
            <div class="flex-1">
              <p class="font-medium text-gray-700">{{ u.username }}</p>
              <p class="text-xs truncate" :class="u.status === 'online' ? 'text-green-500' : 'text-red-500'">{{ u.status === 'online' ? '在线' : '离线' }}</p>
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
            <div class="relative">
              <div class="w-10 h-10 rounded-full bg-gradient-to-br from-purple-400 to-purple-600 flex items-center justify-center text-white font-bold">
                {{ g.name.charAt(0) }}
              </div>
              <div v-if="unreadCounts[g.id] > 0" class="absolute -top-1 -right-1 w-5 h-5 bg-red-500 rounded-full flex items-center justify-center text-white text-xs font-bold border-2 border-white">
                {{ unreadCounts[g.id] > 99 ? '99+' : unreadCounts[g.id] }}
              </div>
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
            <p class="text-xs" :class="selectedContact.status === 'online' ? 'text-green-500' : 'text-red-500'">{{ selectedContact.status === 'online' ? '在线' : '离线' }}</p>
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
      <div class="flex-1 bg-white/60 backdrop-blur-sm overflow-y-auto p-4 space-y-4" ref="messagesContainer" @scroll="handleScroll">
        <!-- 加载更多提示 -->
        <div v-if="hasMoreMessages && loadingMore" class="text-center py-2">
          <div class="inline-block w-5 h-5 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
          <span class="text-xs text-gray-400 ml-2">加载更多...</span>
        </div>
        <div v-else-if="hasMoreMessages && !loadingMore" class="text-center py-2">
          <button @click="loadMoreMessages" class="text-xs text-blue-500 hover:text-blue-600 transition-colors">
            加载更多消息
          </button>
        </div>
        
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
            <div class="max-w-[70%] min-w-0">
              <p v-if="chatType === 'group'" class="text-xs text-gray-500 mb-1">{{ getSenderName(message.sender_id) }}</p>
              <div class="bg-white rounded-2xl rounded-tl-none px-4 py-2 shadow-sm message-bubble">
                <p class="text-gray-700 break-words overflow-wrap-anywhere">{{ message.content }}</p>
              </div>
              <span class="text-xs text-gray-400 mt-1 block">{{ formatTime(message.timestamp) }}</span>
            </div>
          </template>
          <template v-else>
            <div class="max-w-[70%] min-w-0">
              <div class="bg-gradient-to-br from-pink-500 to-pink-600 rounded-2xl rounded-tr-none px-4 py-2 shadow-sm message-bubble">
                <p class="text-white break-words overflow-wrap-anywhere">{{ message.content }}</p>
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
        
        <div class="flex gap-3 items-end">
          <button 
            @click="showEmojiPicker = !showEmojiPicker"
            class="w-10 h-10 rounded-full bg-pink-100 text-pink-500 flex items-center justify-center hover:bg-pink-200 transition-colors relative flex-shrink-0"
          >
            😊
          </button>
          <button class="w-10 h-10 rounded-full bg-pink-100 text-pink-500 flex items-center justify-center hover:bg-pink-200 transition-colors flex-shrink-0">
            📎
          </button>
          <textarea 
            ref="messageInputRef"
            v-model="newMessage"
            @keydown.enter.exact.prevent="sendMessage"
            @input="autoResizeTextarea"
            placeholder="输入消息..." 
            rows="1"
            class="flex-1 px-4 py-2.5 rounded-2xl border-2 border-pink-100 focus:border-pink-300 focus:outline-none transition-colors resize-none overflow-y-auto message-textarea"
          ></textarea>
          <button 
            @click="sendMessage"
            class="px-6 py-2.5 rounded-full bg-gradient-to-r from-pink-500 to-pink-600 text-white font-medium hover:from-pink-600 hover:to-pink-700 transition-all hover:shadow-lg flex-shrink-0 self-end"
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
          <p class="text-sm" :class="selectedContact.status === 'online' ? 'text-green-500' : 'text-red-500'">
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
              <p class="text-xs" :class="member.status === 'online' ? 'text-green-500' : 'text-red-500'">{{ member.status === 'online' ? '在线' : '离线' }}</p>
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

    <!-- 管理员面板 -->
    <div v-if="showAdminPanel" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-3xl p-6 w-[800px] max-h-[90vh] shadow-2xl flex flex-col">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-bold text-gray-700 text-lg">管理员面板</h3>
          <button 
            @click="showAdminPanel = false"
            class="text-gray-500 hover:text-gray-700 transition-colors"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        
        <!-- 标签页 -->
        <div class="flex border-b mb-4">
          <button 
            @click="adminTab = 'users'"
            :class="adminTab === 'users' ? 'border-b-2 border-blue-500 text-blue-600' : 'text-gray-500'"
            class="flex-1 py-2 font-medium transition-colors"
          >
            用户管理
          </button>
          <button 
            @click="adminTab = 'messages'"
            :class="adminTab === 'messages' ? 'border-b-2 border-blue-500 text-blue-600' : 'text-gray-500'"
            class="flex-1 py-2 font-medium transition-colors"
          >
            聊天记录管理
          </button>
        </div>
        
        <!-- 用户管理 -->
        <div v-if="adminTab === 'users'" class="flex-1 flex flex-col">
          <div class="flex gap-2 mb-4">
            <button 
              @click="loadAllUsers"
              class="flex-1 py-2 rounded-full bg-gradient-to-r from-blue-500 to-blue-600 text-white font-medium hover:from-blue-600 hover:to-blue-700 transition-all"
            >
              刷新用户列表
            </button>
          </div>
          <div class="flex gap-2 mb-4">
            <button 
              @click="deleteAllUsers"
              class="flex-1 py-2 rounded-full bg-gradient-to-r from-red-500 to-red-600 text-white font-medium hover:from-red-600 hover:to-red-700 transition-all"
            >
              删除所有非管理员用户
            </button>
          </div>
          <div class="flex gap-2 mb-4">
            <button 
              @click="deleteAllGroups"
              class="flex-1 py-2 rounded-full bg-gradient-to-r from-orange-500 to-orange-600 text-white font-medium hover:from-orange-600 hover:to-orange-700 transition-all"
            >
              删除所有群聊
            </button>
            <button 
              @click="deleteAllMessages"
              class="flex-1 py-2 rounded-full bg-gradient-to-r from-yellow-500 to-yellow-600 text-white font-medium hover:from-yellow-600 hover:to-yellow-700 transition-all"
            >
              删除所有对话消息
            </button>
          </div>
          
          <div class="flex-1 overflow-y-auto">
            <h4 class="font-medium text-gray-700 mb-2">所有用户 ({{ allUsers.length }})</h4>
            <div class="space-y-2">
              <div 
                v-for="u in allUsers" 
                :key="u.id"
                class="flex items-center justify-between p-3 rounded-xl bg-gray-50 hover:bg-gray-100 transition-colors"
              >
                <div class="flex items-center gap-3">
                  <div class="w-10 h-10 rounded-full bg-gradient-to-br from-purple-400 to-purple-600 flex items-center justify-center text-white font-bold">
                    {{ u.username.charAt(0).toUpperCase() }}
                  </div>
                  <div>
                    <p class="font-medium text-gray-700">{{ u.username }} <span v-if="u.is_admin" class="text-xs text-blue-500">(管理员)</span></p>
                    <p class="text-xs text-gray-500">{{ u.email }}</p>
                  </div>
                </div>
                <div class="flex items-center gap-2">
                  <span class="text-xs px-2 py-1 rounded-full" :class="u.status === 'online' ? 'bg-green-100 text-green-600' : 'bg-red-100 text-red-600'">
                    {{ u.status === 'online' ? '在线' : '离线' }}
                  </span>
                  <button 
                    @click="viewUserChatHistory(u.id)"
                    class="px-3 py-1 rounded-full bg-blue-100 text-blue-600 text-xs font-medium hover:bg-blue-200 transition-colors"
                  >
                    查看聊天
                  </button>
                  <button 
                    @click="clearUserMessages(u.id)"
                    class="px-3 py-1 rounded-full bg-orange-100 text-orange-600 text-xs font-medium hover:bg-orange-200 transition-colors"
                  >
                    清空聊天
                  </button>
                  <button 
                    v-if="!u.is_admin && u.id !== currentUserId"
                    @click="deleteUser(u.id)"
                    class="px-3 py-1 rounded-full bg-red-100 text-red-600 text-xs font-medium hover:bg-red-200 transition-colors"
                  >
                    删除
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 聊天记录管理 -->
        <div v-if="adminTab === 'messages'" class="flex-1 flex flex-col">
          <!-- 类型筛选 -->
          <div class="flex gap-2 mb-4">
            <button 
              @click="chatTypeFilter = 'all'; conversationPage = 1; loadConversations()"
              :class="chatTypeFilter === 'all' ? 'bg-blue-500 text-white' : 'bg-gray-100 text-gray-600'"
              class="flex-1 py-2 rounded-full font-medium transition-all"
            >
              全部
            </button>
            <button 
              @click="chatTypeFilter = 'private'; conversationPage = 1; loadConversations()"
              :class="chatTypeFilter === 'private' ? 'bg-blue-500 text-white' : 'bg-gray-100 text-gray-600'"
              class="flex-1 py-2 rounded-full font-medium transition-all"
            >
              私人对话
            </button>
            <button 
              @click="chatTypeFilter = 'group'; conversationPage = 1; loadConversations()"
              :class="chatTypeFilter === 'group' ? 'bg-blue-500 text-white' : 'bg-gray-100 text-gray-600'"
              class="flex-1 py-2 rounded-full font-medium transition-all"
            >
              群聊
            </button>
          </div>

          <div class="flex gap-2 mb-4">
            <button 
              @click="loadConversations"
              class="flex-1 py-2 rounded-full bg-gradient-to-r from-blue-500 to-blue-600 text-white font-medium hover:from-blue-600 hover:to-blue-700 transition-all"
            >
              刷新
            </button>
            <button 
              @click="deleteAllMessages"
              class="flex-1 py-2 rounded-full bg-gradient-to-r from-red-500 to-red-600 text-white font-medium hover:from-red-600 hover:to-red-700 transition-all"
            >
              删除所有消息
            </button>
          </div>
          
          <!-- 对话列表 -->
          <div v-if="!viewingConversation" class="flex-1 overflow-y-auto">
            <h4 class="font-medium text-gray-700 mb-2">对话列表 (共 {{ totalConversations }} 个)</h4>
            <div class="space-y-2">
              <div 
                v-for="conv in conversations" 
                :key="conv.id"
                @click="viewConversation(conv)"
                class="flex items-center gap-3 p-3 rounded-xl bg-gray-50 hover:bg-gray-100 transition-colors cursor-pointer"
              >
                <!-- 图标 -->
                <div class="w-10 h-10 rounded-full flex items-center justify-center text-white font-bold"
                  :class="conv.chat_type === 'group' ? 'bg-gradient-to-br from-green-400 to-green-600' : 'bg-gradient-to-br from-blue-400 to-blue-600'">
                  {{ conv.chat_type === 'group' ? 'G' : (conv.other_usernames || '?').charAt(0).toUpperCase() }}
                </div>
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-xs font-medium text-gray-700">
                      {{ conv.chat_type === 'group' ? conv.group_name : conv.other_usernames }}
                    </span>
                    <span class="text-xs px-2 py-0.5 rounded-full"
                      :class="conv.chat_type === 'group' ? 'bg-green-100 text-green-600' : 'bg-blue-100 text-blue-600'">
                      {{ conv.chat_type === 'group' ? '群聊' : '私聊' }}
                    </span>
                    <span class="text-xs text-gray-400 ml-auto">{{ formatConversationTime(conv.last_message_time) }}</span>
                  </div>
                  <p class="text-sm text-gray-500 truncate">{{ conv.last_message }}</p>
                </div>
                <div class="flex flex-col items-end gap-1">
                  <span class="text-xs text-gray-400">{{ conv.message_count }} 条消息</span>
                  <button 
                    @click.stop="clearConversation(conv)"
                    class="px-2 py-1 rounded-full bg-red-100 text-red-600 text-xs font-medium hover:bg-red-200 transition-colors"
                  >
                    清空
                  </button>
                </div>
              </div>
            </div>
            
            <!-- 分页 -->
            <div v-if="totalConversations > conversationPageSize" class="flex justify-center gap-2 mt-4">
              <button 
                @click="conversationPage--"
                :disabled="conversationPage <= 1"
                class="px-4 py-2 rounded-full bg-gray-100 text-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                上一页
              </button>
              <span class="px-4 py-2 text-gray-600">第 {{ conversationPage }} 页</span>
              <button 
                @click="conversationPage++"
                :disabled="conversationPage * conversationPageSize >= totalConversations"
                class="px-4 py-2 rounded-full bg-gray-100 text-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                下一页
              </button>
            </div>
          </div>
          
          <!-- 对话消息详情 -->
          <div v-else class="flex-1 flex flex-col">
            <div class="flex items-center gap-2 mb-4">
              <button 
                @click="viewingConversation = null"
                class="px-3 py-1 rounded-full bg-gray-100 text-gray-600 text-sm hover:bg-gray-200 transition-colors"
              >
                ← 返回
              </button>
              <h4 class="font-medium text-gray-700">
                {{ viewingConversation.chat_type === 'group' ? viewingConversation.group_name : viewingConversation.other_usernames }}
              </h4>
              <span class="text-xs text-gray-400">({{ viewingConversation.message_count }} 条消息)</span>
            </div>
            
            <div class="flex-1 overflow-y-auto max-h-[50vh]">
              <div class="space-y-2">
                <div 
                  v-for="msg in conversationMessages" 
                  :key="msg.id"
                  class="flex items-start gap-3 p-3 rounded-xl bg-gray-50 hover:bg-gray-100 transition-colors"
                  :class="{ 'border-l-4 border-blue-500': selectedMessages.includes(msg.id) }"
                >
                  <input 
                    type="checkbox" 
                    :value="msg.id"
                    v-model="selectedMessages"
                    class="w-4 h-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500 mt-1"
                  />
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2 mb-1">
                      <span class="text-xs font-medium text-blue-600">{{ msg.sender_name || getUserName(msg.sender_id) }}</span>
                      <span class="text-xs text-gray-400 ml-auto">{{ formatMessageTime(msg.timestamp) }}</span>
                    </div>
                    <p class="text-sm text-gray-700">{{ msg.content }}</p>
                  </div>
                </div>
              </div>
              
              <!-- 分页 -->
              <div v-if="conversationMessageTotal > conversationPageSize" class="flex justify-center gap-2 mt-4 pb-2">
                <button 
                  @click="conversationMsgPage--"
                  :disabled="conversationMsgPage <= 1"
                  class="px-4 py-2 rounded-full bg-gray-100 text-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  上一页
                </button>
                <span class="px-4 py-2 text-gray-600">第 {{ conversationMsgPage }} 页</span>
                <button 
                  @click="conversationMsgPage++"
                  :disabled="conversationMsgPage * conversationPageSize >= conversationMessageTotal"
                  class="px-4 py-2 rounded-full bg-gray-100 text-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  下一页
                </button>
              </div>
            </div>
            
            <div class="flex gap-2 mt-4">
              <button 
                @click="deleteSelectedMessages"
                :disabled="selectedMessages.length === 0"
                class="flex-1 py-2 rounded-full bg-gradient-to-r from-red-500 to-red-600 text-white font-medium hover:from-red-600 hover:to-red-700 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
              >
                删除选中 ({{ selectedMessages.length }})
              </button>
            </div>
          </div>
        </div>
        
        <!-- 用户聊天历史弹窗 -->
        <div v-if="showUserChatModal" class="fixed inset-0 bg-black/50 flex items-center justify-center z-60">
          <div class="bg-white rounded-3xl p-6 w-[600px] max-h-[80vh] shadow-2xl flex flex-col">
            <div class="flex items-center justify-between mb-4">
              <h3 class="font-bold text-gray-700 text-lg">用户聊天历史 - {{ currentViewUserName }}</h3>
              <button 
                @click="showUserChatModal = false"
                class="text-gray-500 hover:text-gray-700 transition-colors"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
            
            <div class="flex-1 overflow-y-auto">
              <div class="space-y-2">
                <div 
                  v-for="msg in userChatHistory" 
                  :key="msg.id"
                  class="p-3 rounded-xl"
                  :class="msg.sender_id === currentViewUserId ? 'bg-blue-50' : 'bg-gray-50'"
                >
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-xs font-medium text-blue-600">{{ getUserName(msg.sender_id) }}</span>
                    <span class="text-xs text-gray-400 ml-auto">{{ formatMessageTime(msg.timestamp) }}</span>
                  </div>
                  <p class="text-sm text-gray-700">{{ msg.content }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed, watch } from 'vue'
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
const userStatus = ref('online')
const isAdmin = ref(false)
const users = ref<Contact[]>([])
const selectedContact = ref<Contact | null>(null)
const selectedGroup = ref<Group | null>(null)
const groupMembers = ref<GroupMember[]>([])
const messages = ref<Message[]>([])
const groups = ref<Group[]>([])
const showAdminPanel = ref(false)
const adminTab = ref<'users' | 'messages'>('users')
const allUsers = ref<any[]>([])
const allMessages = ref<any[]>([])
const totalMessages = ref(0)
const messagePage = ref(1)
const pageSize = ref(50)
const messageSearchUserId = ref('')
const selectedMessages = ref<string[]>([])
const showUserChatModal = ref(false)
const currentViewUserId = ref('')
const currentViewUserName = ref('')
const userChatHistory = ref<any[]>([])

// 消息分页相关
const messagePageCurrent = ref(1)
const hasMoreMessages = ref(false)
const loadingMore = ref(false)
const targetIdForMessages = ref('')

// 未读消息相关
const unreadCounts = ref<Record<string, number>>({})

// 对话列表相关
const chatTypeFilter = ref<'all' | 'private' | 'group'>('all')
const conversations = ref<any[]>([])
const totalConversations = ref(0)
const conversationPage = ref(1)
const conversationPageSize = ref(50)
const viewingConversation = ref<any>(null)
const conversationMessages = ref<any[]>([])
const conversationMsgPage = ref(1)
const conversationMessageTotal = ref(0)

const newMessage = ref('')
const currentUserId = ref('')
const messagesContainer = ref<HTMLElement | null>(null)
const searchQuery = ref('')
const searchResults = ref<User[]>([])
const searchType = ref<'username' | 'email'>('username')
const isSearching = ref(false)
const showEmojiPicker = ref(false)
const messageInputRef = ref<HTMLTextAreaElement | null>(null)

// 自动调整 textarea 高度
const autoResizeTextarea = () => {
  const textarea = messageInputRef.value
  if (!textarea) return
  
  textarea.style.height = 'auto'
  textarea.style.height = Math.min(textarea.scrollHeight, 120) + 'px'
}

const resetTextareaHeight = () => {
  const textarea = messageInputRef.value
  if (textarea) {
    textarea.style.height = 'auto'
  }
}
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
  messagePageCurrent.value = 1
  hasMoreMessages.value = false
  targetIdForMessages.value = contact.id
  // 清除未读计数
  unreadCounts.value[contact.id] = 0
  loadMessages(contact.id)
}

const selectGroup = async (group: Group) => {
  chatType.value = 'group'
  selectedGroup.value = group
  selectedContact.value = null
  messages.value = []
  // 清除未读计数
  unreadCounts.value[group.id] = 0
  
  await loadGroupMembers(group.id)
  loadMessages(group.id)
  loadFriends() // 加载好友列表，而不是所有用户
}

const loadMessages = async (targetId: string, page: number = 1, append: boolean = false) => {
  try {
    const params = chatType.value === 'group' 
      ? { group_id: targetId, limit: 50, page }
      : { contact_id: targetId, limit: 50, page }
    
    const response = await api.get('/messages', { params })
    
    // 判断是否还有更多消息
    hasMoreMessages.value = response.data.length >= 50
    
    if (append) {
      // 保存当前滚动位置
      const container = messagesContainer.value
      const oldScrollHeight = container?.scrollHeight || 0
      
      messages.value = [...response.data, ...messages.value]
      messagePageCurrent.value = page
      
      // 恢复滚动位置到新消息底部
      nextTick(() => {
        if (container) {
          container.scrollTop = container.scrollHeight - oldScrollHeight
        }
      })
    } else {
      messages.value = response.data
      messagePageCurrent.value = page
      targetIdForMessages.value = targetId
      scrollToBottom()
    }
  } catch (error) {
    console.error('Failed to load messages:', error)
    messages.value = []
  }
}

const loadMoreMessages = async () => {
  if (loadingMore.value || !hasMoreMessages.value) return
  
  loadingMore.value = true
  try {
    await loadMessages(targetIdForMessages.value, messagePageCurrent.value + 1, true)
  } finally {
    loadingMore.value = false
  }
}

// 滚动事件处理
const handleScroll = (event: Event) => {
  const target = event.target as HTMLElement
  // 当滚动到顶部时加载更多
  if (target.scrollTop < 50 && hasMoreMessages.value && !loadingMore.value) {
    loadMoreMessages()
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

const handleBeforeUnload = async (event: BeforeUnloadEvent) => {
  try {
    const token = localStorage.getItem('token')
    if (token) {
      await fetch('/api/users/status', {
        method: 'PUT',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ status: 'offline' })
      })
    }
  } catch (error) {
    console.error('Failed to update status on close:', error)
  }
  return undefined
}

const loadAllUsers = async () => {
  try {
    const response = await api.get('/admin/users')
    allUsers.value = response.data
  } catch (error) {
    console.error('Failed to load users:', error)
  }
}

const deleteUser = async (userId: string) => {
  if (!confirm('确定要删除该用户吗？此操作不可恢复。')) return
  
  try {
    await api.delete(`/admin/users/${userId}`)
    await loadAllUsers()
    await loadFriends()
  } catch (error: any) {
    alert(error.response?.data?.error || '删除用户失败')
  }
}

const deleteAllUsers = async () => {
  if (!confirm('警告：此操作将删除所有非管理员用户（除了您自己），此操作不可恢复。是否继续？')) return
  
  try {
    const response = await api.delete('/admin/users/delete-all')
    alert(response.data.message)
    await loadAllUsers()
    await loadFriends()
    await loadGroups()
  } catch (error: any) {
    alert(error.response?.data?.error || '删除用户失败')
  }
}

const deleteAllGroups = async () => {
  if (!confirm('警告：此操作将删除所有群聊及成员关系，此操作不可恢复。是否继续？')) return
  
  try {
    const response = await api.delete('/admin/groups/delete-all')
    alert(response.data.message)
    await loadGroups()
    selectedGroup.value = null
    groupMembers.value = []
  } catch (error: any) {
    alert(error.response?.data?.error || '删除群聊失败')
  }
}

const deleteAllMessages = async () => {
  if (!confirm('警告：此操作将删除所有对话消息，此操作不可恢复。是否继续？')) return
  
  try {
    const response = await api.delete('/admin/messages/delete-all')
    alert(response.data.message)
    messages.value = []
  } catch (error: any) {
    alert(error.response?.data?.error || '删除消息失败')
  }
}

const deleteMessage = async (messageId: string) => {
  if (!confirm('确定要删除该消息吗？此操作不可恢复。')) return
  
  try {
    await api.delete(`/admin/messages/${messageId}`)
  } catch (error: any) {
    alert(error.response?.data?.error || '删除消息失败')
  }
}

const clearChatHistory = () => {
  if (confirm('确定要清空聊天记录吗？此操作不可恢复。')) {
    messages.value = []
  }
}

const getUserName = (userId: string): string => {
  const u = allUsers.value.find(u => u.id === userId)
  return u ? u.username : (userId === currentUserId.value ? '我' : userId.substring(0, 8))
}

const formatMessageTime = (timestamp: string): string => {
  const date = new Date(timestamp)
  return `${date.getMonth() + 1}/${date.getDate()} ${date.getHours()}:${String(date.getMinutes()).padStart(2, '0')}`
}

const formatConversationTime = (timestamp: string): string => {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const hours = diff / (1000 * 60 * 60)
  if (hours < 1) {
    const mins = Math.floor(diff / (1000 * 60))
    return `${mins}分钟前`
  } else if (hours < 24) {
    return `${Math.floor(hours)}小时前`
  } else {
    return `${date.getMonth() + 1}月${date.getDate()}日`
  }
}

const loadConversations = async () => {
  try {
    const params: any = { 
      page: conversationPage.value, 
      page_size: conversationPageSize.value 
    }
    if (chatTypeFilter.value !== 'all') {
      params.chat_type = chatTypeFilter.value
    }
    const response = await api.get('/admin/conversations', { params })
    conversations.value = response.data.conversations
    totalConversations.value = response.data.total
  } catch (error) {
    console.error('Failed to load conversations:', error)
  }
}

const viewConversation = async (conv: any) => {
  viewingConversation.value = conv
  conversationMsgPage.value = 1
  selectedMessages.value = []
  
  try {
    if (conv.chat_type === 'group') {
      const response = await api.get(`/admin/groups/${conv.group_id}/messages`, {
        params: { page: conversationMsgPage.value, page_size: 50 }
      })
      conversationMessages.value = response.data.messages
      conversationMessageTotal.value = response.data.total
    } else {
      // For private chats, load messages involving both users
      const userIds = conv.user_ids
      if (userIds && userIds.length >= 2) {
        const response = await api.get('/admin/messages', {
          params: {
            user_id: userIds[0],
            page: conversationMsgPage.value,
            page_size: 50
          }
        })
        // Filter to only show messages between the two users
        conversationMessages.value = response.data.messages.filter(
          (msg: any) => msg.sender_id === userIds[0] || msg.receiver_id === userIds[0]
        )
        conversationMessageTotal.value = response.data.total
      }
    }
  } catch (error) {
    console.error('Failed to load conversation messages:', error)
  }
}

const clearConversation = async (conv: any) => {
  if (!confirm(`确定要清空与 ${conv.group_name || conv.other_usernames} 的聊天记录吗？此操作不可恢复。`)) return
  
  try {
    if (conv.chat_type === 'group') {
      const response = await api.delete(`/admin/groups/${conv.group_id}/messages/clear`)
      alert(response.data.message)
    } else {
      const userIds = conv.user_ids
      if (userIds && userIds.length >= 2) {
        await api.delete(`/admin/users/${userIds[0]}/messages/clear`)
        alert('已清空该对话的消息')
      }
    }
    await loadConversations()
  } catch (error: any) {
    alert(error.response?.data?.error || '清空消息失败')
  }
}

const loadAllMessages = async () => {
  try {
    const params: any = { page: messagePage.value, page_size: pageSize.value }
    if (messageSearchUserId.value) {
      params.user_id = messageSearchUserId.value
    }
    const response = await api.get('/admin/messages', { params })
    allMessages.value = response.data.messages
    totalMessages.value = response.data.total
    selectedMessages.value = []
  } catch (error) {
    console.error('Failed to load messages:', error)
  }
}

const deleteSelectedMessages = async () => {
  if (selectedMessages.value.length === 0) return
  if (!confirm(`确定要删除选中的 ${selectedMessages.value.length} 条消息吗？此操作不可恢复。`)) return
  
  try {
    await api.delete('/admin/messages/delete-selected', {
      data: { message_ids: selectedMessages.value }
    })
    await loadAllMessages()
  } catch (error: any) {
    alert(error.response?.data?.error || '删除消息失败')
  }
}

const viewUserChatHistory = async (userId: string) => {
  const u = allUsers.value.find(u => u.id === userId)
  currentViewUserId.value = userId
  currentViewUserName.value = u ? u.username : userId
  showUserChatModal.value = true
  
  try {
    const response = await api.get(`/admin/users/${userId}/messages`, {
      params: { page: 1, page_size: 100 }
    })
    userChatHistory.value = response.data.messages
  } catch (error) {
    console.error('Failed to load chat history:', error)
  }
}

const clearUserMessages = async (userId: string) => {
  const u = allUsers.value.find(u => u.id === userId)
  if (!confirm(`确定要清空 ${u?.username} 的所有聊天记录吗？此操作不可恢复。`)) return
  
  try {
    const response = await api.delete(`/admin/users/${userId}/messages/clear`)
    alert(response.data.message)
  } catch (error: any) {
    alert(error.response?.data?.error || '清空消息失败')
  }
}

const removeFriend = async () => {
  if (!selectedContact.value) return
  
  if (confirm(`确定要删除好友 ${selectedContact.value.username} 吗？`)) {
    try {
      // 先删除聊天记录
      await api.delete('/messages/chat', {
        data: { contact_id: selectedContact.value.id }
      })
      
      // 再删除好友关系
      await api.delete('/friends/remove', {
        data: { friend_id: selectedContact.value.id }
      })
      
      // 重新加载好友列表
      await loadFriends()
      
      // 清空当前聊天消息
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

watch(adminTab, (newTab) => {
  if (newTab === 'messages') {
    loadAllUsers()
    loadConversations()
  }
})

watch(messagePage, () => {
  loadAllMessages()
})

watch(messageSearchUserId, () => {
  messagePage.value = 1
  loadAllMessages()
})

watch(conversationPage, () => {
  loadConversations()
})

watch(conversationMsgPage, () => {
  if (viewingConversation.value) {
    viewConversation(viewingConversation.value)
  }
})

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
    await api.post('/messages', {
      receiver_id: receiverId,
      content: newMessage.value,
      message_type: 'text'
    })
    // 所有消息（私聊和群聊）都通过 WebSocket 通知添加，避免重复
    newMessage.value = ''
    resetTextareaHeight()
  } catch (error) {
    console.error('Failed to send message:', error)
  }
}

const connectWebSocket = () => {
  const userStr = localStorage.getItem('user')
  if (!userStr) return
  
  const userData = JSON.parse(userStr)
  
  // 动态获取 WebSocket 地址：根据当前页面地址自动切换
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
  const host = window.location.hostname
  const wsUrl = `${protocol}//${host}:8080/ws?user_id=${userData.id}`
  
  console.log('Connecting to WebSocket:', wsUrl)
  ws = new WebSocket(wsUrl)
  
  ws.onopen = () => {
    console.log('WebSocket connected')
    // WebSocket 连接成功后更新当前用户状态为在线
    userStatus.value = 'online'
    // 重新加载好友列表，确保在线状态正确
    loadFriends()
  }
  
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data)
    if (data.type === 'status_update') {
      // 更新私聊用户列表
      users.value = users.value.map(u => 
        u.id === data.user_id ? { ...u, status: data.status } : u
      )
      // 更新好友列表
      friends.value = friends.value.map(f =>
        f.id === data.user_id ? { ...f, status: data.status } : f
      )
      // 更新群成员列表
      groupMembers.value = groupMembers.value.map(m =>
        m.id === data.user_id ? { ...m, status: data.status } : m
      )
      // 更新管理员面板用户列表
      allUsers.value = allUsers.value.map(u =>
        u.id === data.user_id ? { ...u, status: data.status } : u
      )
      // 更新选中联系人状态
      if (selectedContact.value && selectedContact.value.id === data.user_id) {
        selectedContact.value = { ...selectedContact.value, status: data.status }
      }
      // 更新对话列表中的用户状态
      conversations.value = conversations.value.map((conv: any) => {
        if (conv.user_ids && conv.user_ids.includes(data.user_id)) {
          return { ...conv, last_message_time: new Date().toISOString() }
        }
        return conv
      })
    } else if (data.type === 'new_message') {
      const newMsg = data.data
      const exists = messages.value.some(m => m.id === newMsg.id)
      if (!exists) {
        // 私聊消息：当前正在查看该对话（无论发送者还是接收者）
        if (chatType.value === 'private' && selectedContact.value && 
            (newMsg.sender_id === selectedContact.value.id || newMsg.receiver_id === selectedContact.value.id) &&
            (newMsg.sender_id === currentUserId.value || newMsg.receiver_id === currentUserId.value)) {
          messages.value.push(newMsg)
          scrollToBottom()
        // 群聊消息：当前正在查看该群
        } else if (chatType.value === 'group' && selectedGroup.value &&
                   newMsg.receiver_id === selectedGroup.value.id) {
          messages.value.push(newMsg)
          scrollToBottom()
        } else {
          // 不是当前对话，增加未读计数
          if (newMsg.receiver_id && typeof newMsg.receiver_id === 'string' && newMsg.receiver_id.startsWith('group_')) {
            // 群聊消息
            unreadCounts.value[newMsg.receiver_id] = (unreadCounts.value[newMsg.receiver_id] || 0) + 1
          } else if (newMsg.sender_id && newMsg.sender_id !== currentUserId.value) {
            // 私聊消息
            unreadCounts.value[newMsg.sender_id] = (unreadCounts.value[newMsg.sender_id] || 0) + 1
          }
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
    userStatus.value = user.value.status || 'online'
    isAdmin.value = user.value.is_admin || false
  }
  
  window.addEventListener('beforeunload', handleBeforeUnload)
  
  loadFriends()
  loadGroups()
  connectWebSocket()
})

onUnmounted(() => {
  window.removeEventListener('beforeunload', handleBeforeUnload)
  if (ws) {
    ws.close()
  }
})
</script>

<style scoped>
.message-textarea {
  font-size: 1rem;
  line-height: 1.5;
  min-height: 42px;
  max-height: 120px;
  font-family: inherit;
}

.message-textarea::-webkit-scrollbar {
  width: 6px;
}

.message-textarea::-webkit-scrollbar-track {
  background: transparent;
}

.message-textarea::-webkit-scrollbar-thumb {
  background-color: #f9a8d4;
  border-radius: 3px;
}

.message-textarea::-webkit-scrollbar-thumb:hover {
  background-color: #f472b6;
}

.message-bubble {
  word-wrap: break-word;
  word-break: break-word;
  overflow-wrap: anywhere;
}

.message-bubble p {
  white-space: pre-wrap;
}
</style>