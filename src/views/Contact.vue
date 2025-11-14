<template>
  <div>
    <!-- Hero Section -->
  <section class="bg-gradient-to-r from-primary-600 to-primary-700 text-white">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
        <div class="text-center">
          <h1 class="text-4xl lg:text-6xl font-bold mb-6">
            Hubungi indoprint.id
          </h1>
          <p class="text-xl lg:text-2xl text-primary-100">
            Konsultasi & Pemesanan Sablon dan Merchandise Custom
          </p>
        </div>
      </div>
    </section>

    <!-- Contact Info & Maps Section -->
  <section class="py-20 bg-gray-900">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
          <!-- Contact Information -->
          <div>
            <div class="bg-gray-800 p-8 rounded-lg shadow-lg">
              <h2 class="text-3xl font-bold text-white mb-6">
                Kontak & Lokasi
              </h2>
              <p class="text-gray-300 mb-6">
                indoprint.id siap membantu kebutuhan sablon, cetak, dan merchandise custom untuk bisnis, komunitas, maupun personal. Hubungi kami untuk konsultasi desain, pemesanan, atau informasi lebih lanjut.
              </p>
              
              <div class="space-y-6">
                <div class="flex items-start">
                  <div class="text-primary-600 text-xl mr-4 mt-1">📞</div>
                  <div>
                    <h3 class="font-semibold text-white mb-1">Telepon</h3>
                    <p class="text-gray-300">+6281234567890</p>
                  </div>
                </div>
                
                <div class="flex items-start">
                  <div class="text-primary-600 text-xl mr-4 mt-1">✉️</div>
                  <div>
                    <h3 class="font-semibold text-white mb-1">Email</h3>
                    <p class="text-gray-300">info@indoprint.id</p>
                  </div>
                </div>
                
                <div class="flex items-start">
                  <div class="text-primary-600 text-xl mr-4 mt-1">📍</div>
                  <div>
                    <h3 class="font-semibold text-white mb-1">Alamat</h3>
                    <p class="text-gray-300">
                      Jl. Sablon Kreatif No. 10<br>
                      Jakarta, Indonesia
                    </p>
                  </div>
                </div>
                
                <div class="flex items-start">
                  <div class="text-primary-600 text-xl mr-4 mt-1">🕒</div>
                  <div>
                    <h3 class="font-semibold text-white mb-1">Jam Operasional</h3>
                    <p class="text-gray-300">
                      Senin - Jumat: 08.00 - 17.00<br>
                      Sabtu: 08.00 - 12.00<br>
                      Minggu: Libur
                    </p>
                  </div>
                </div>
              </div>
              
              <div class="mt-8">
                <a 
                  href="https://maps.google.com/?q=Jl.+Sablon+Kreatif+No.+10+Jakarta"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="bg-primary-600 text-white px-6 py-3 rounded-lg font-semibold hover:bg-primary-700 transition-colors duration-200 inline-flex items-center"
                >
                  <span class="mr-2">🗺️</span>
                  Open in Google Maps
                </a>
              </div>
            </div>
          </div>
          
          <!-- Map -->
          <div>
            <div class="bg-gray-800 p-4 rounded-lg shadow-lg h-full">
              <div class="relative w-full h-96 lg:h-full min-h-96">
                <iframe 
                  src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3966.521260322283!2d106.95161731476933!3d-6.194741395493371!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x2e698b85e6b4c2f1%3A0x3b4b4b4b4b4b4b4b!2sKomplek%20Pergudangan%20Bizhub%2052!5e0!3m2!1sen!2sid!4v1234567890"
                  width="100%" 
                  height="100%" 
                  style="border:0; border-radius: 8px;" 
                  allowfullscreen="" 
                  loading="lazy"
                  class="absolute inset-0 w-full h-full"
                ></iframe>
              </div>
              <div class="mt-4 p-4 bg-gray-700 rounded-lg">
                <div class="flex items-center mb-2">
                  <div class="text-yellow-400">★★★★★</div>
                  <span class="ml-2 font-semibold text-white">4.6</span>
                  <span class="ml-1 text-gray-300">(38 reviews)</span>
                </div>
                <p class="text-sm text-gray-300">
                  <strong>indoprint.id</strong><br>
                  Jl. Sablon Kreatif No. 10, Jakarta
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Contact Form -->
  <section class="py-20 bg-gray-900">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-12">
            <h2 class="text-3xl lg:text-4xl font-bold text-white mb-4">
                Kirim Pesan ke Kami
              </h2>
              <p class="text-xl text-gray-300">
                Punya pertanyaan seputar layanan sablon atau merchandise? Silakan isi form berikut, tim kami akan segera menghubungi Anda.
              </p>
        </div>
        
        <form @submit.prevent="submitForm" class="space-y-6">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label for="name" class="block text-sm font-medium text-gray-300 mb-2">
                Name *
              </label>
              <input 
                type="text" 
                id="name" 
                v-model="form.name"
                required
                class="w-full px-4 py-3 bg-gray-800 border border-gray-700 text-white rounded-lg focus:ring-2 focus:ring-primary-600 focus:border-transparent"
                placeholder="Your name"
              />
            </div>
            
            <div>
              <label for="email" class="block text-sm font-medium text-gray-300 mb-2">
                Email *
              </label>
              <input 
                type="email" 
                id="email" 
                v-model="form.email"
                required
                class="w-full px-4 py-3 bg-gray-800 border border-gray-700 text-white rounded-lg focus:ring-2 focus:ring-primary-600 focus:border-transparent"
                placeholder="your@email.com"
              />
            </div>
          </div>
          
          <div>
            <label for="phone" class="block text-sm font-medium text-gray-300 mb-2">
              Phone
            </label>
            <input 
              type="tel" 
              id="phone" 
              v-model="form.phone"
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 text-white rounded-lg focus:ring-2 focus:ring-primary-600 focus:border-transparent"
              placeholder="+62 xxx xxxx xxxx"
            />
          </div>
          
          <div>
            <label for="subject" class="block text-sm font-medium text-gray-300 mb-2">
              Subject *
            </label>
            <select 
              id="subject" 
              v-model="form.subject"
              required
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 text-white rounded-lg focus:ring-2 focus:ring-primary-600 focus:border-transparent"
            >
              <option value="">Select a subject</option>
              <option value="general">Pertanyaan Umum</option>
              <option value="sablon">Layanan Sablon</option>
              <option value="merchandise">Custom Merchandise</option>
              <option value="desain">Desain & Konsultasi</option>
              <option value="other">Lainnya</option>
            </select>
          </div>
          
          <div>
            <label for="message" class="block text-sm font-medium text-gray-300 mb-2">
              Message *
            </label>
            <textarea 
              id="message" 
              v-model="form.message"
              rows="5"
              required
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 text-white rounded-lg focus:ring-2 focus:ring-primary-600 focus:border-transparent"
              placeholder="Tell us how we can help you..."
            ></textarea>
          </div>
          
          <div class="text-center">
            <button 
              type="submit"
              :disabled="isSubmitting"
              class="bg-primary-600 text-white px-8 py-3 rounded-lg font-semibold hover:bg-primary-700 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {{ isSubmitting ? 'Sending...' : 'Send Message' }}
            </button>
          </div>
        </form>
        
        <div v-if="showSuccessMessage" class="mt-6 p-4 bg-green-900 border border-green-600 text-green-200 rounded-lg">
          Thank you for your message! We'll get back to you soon.
        </div>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  name: 'Contact',
  data() {
    return {
      form: {
        name: '',
        email: '',
        phone: '',
        subject: '',
        message: ''
      },
      isSubmitting: false,
      showSuccessMessage: false
    }
  },
  methods: {
    async submitForm() {
      this.isSubmitting = true
      
      // Simulate form submission
      setTimeout(() => {
        this.isSubmitting = false
        this.showSuccessMessage = true
        
        // Reset form
        this.form = {
          name: '',
          email: '',
          phone: '',
          subject: '',
          message: ''
        }
        
        // Hide success message after 5 seconds
        setTimeout(() => {
          this.showSuccessMessage = false
        }, 5000)
      }, 1500)
    }
  }
}
</script>