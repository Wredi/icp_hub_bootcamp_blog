<script setup>
import { ref } from 'vue';
import { blog_backend } from 'declarations/blog_backend/index';
let blogs = ref([]);

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;

  const title = target.querySelector('#title').value;
  const content = target.querySelector('#content').value;
  const tags = target.querySelector('#tags').value;
  const splitedTags = tags.split(",");

  await blog_backend.add_blog(title, content, splitedTags);
  await getBlogs();
}

async function getBlogs() {
  const tempBlogs = await blog_backend.get_blogs();
  blogs.value = tempBlogs.map((blog) => {
    return {
      ...blog,
      date: new Date(Number(blog.date) / 1_000_000).toLocaleString()
    }
  });
} 

getBlogs();

</script>

<template>
    <main class="container mx-auto px-6 py-10 bg-gray-900 min-h-screen font-comic">
        <div class="max-w-lg mx-auto bg-gray-800 p-8 rounded-lg shadow-md">
            <h1 class="text-2xl font-bold text-white text-center mb-6">Add a New Blog</h1>
            <form @submit.prevent="handleSubmit" class="space-y-6">
                <div class="flex flex-col">
                    <label for="title" class="text-white text-sm font-medium mb-2">Title</label>
                    <input
                    id="title"
                    type="text"
                    class="px-4 py-2 border border-gray-600 rounded-lg bg-gray-700 text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
                    placeholder="Enter blog title"
                    />
                </div>

                <div class="flex flex-col">
                    <label for="content" class="text-white text-sm font-medium mb-2">Content</label>
                    <textarea
                        id="content"
                        rows="4"
                        class="px-4 py-2 border border-gray-600 rounded-lg bg-gray-700 text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
                        placeholder="Enter blog content"
                        ></textarea>
                </div>

                <div class="flex flex-col">
                    <label for="tags" class="text-white text-sm font-medium mb-2">Tags</label>
                    <input
                    id="tags"
                    type="text"
                    class="px-4 py-2 border border-gray-600 rounded-lg bg-gray-700 text-white focus:ring-2 focus:ring-blue-500 focus:outline-none"
                    placeholder="Enter tags separated by commas"
                    />
                </div>

                <button
                    type="submit"
                    class="w-full py-3 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 focus:ring-4 focus:ring-blue-300 focus:outline-none transition duration-300"
                    >
                    Add Blog
                </button>
            </form>
        </div>

        <div class="mt-10 max-w-lg mx-auto text-white">
            <h2 class="text-xl font-semibold mb-4">Blog Entries</h2>
            <div class="space-y-6">
                <div
                    v-for="(blog, index) in blogs"
                    :key="index"
                    class="p-6 bg-gray-700 rounded-lg shadow-sm"
                    >
                    <h3 class="text-xl font-bold mb-2">{{ blog.title }}</h3>
                    <p class="text-sm text-gray-300 mb-4">{{ blog.date }}</p>
                    <!--<p class="text-gray-200 mb-4">{{ blog.content }}</p>-->
                    <span class="text-gray-200" style="white-space: pre;">{{ blog.content }}</span>
                    <p class="text-sm text-gray-400 mt-4">
                    <strong>Tags:</strong> {{ blog.tags.join(', ') }}
                    </p>
                </div>
            </div>
        </div>
    </main>
</template>
