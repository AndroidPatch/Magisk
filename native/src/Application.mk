APP_BUILD_SCRIPT := src/Android.mk
APP_CFLAGS       := -Wall -O3 -fomit-frame-pointer -fdata-sections -ffunction-sections -fvisibility=hidden -fvisibility-inlines-hidden -funwind-tables -fstack-protector-strong -fno-exceptions -fno-rtti -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-stack-protector -fno-semantic-interposition -U_FORTIFY_SOURCE
APP_CPPFLAGS     := -std=c++23 -O3 -fdata-sections -ffunction-sections -fvisibility=hidden -fvisibility-inlines-hidden -funwind-tables -fstack-protector-strong -fno-exceptions -fno-rtti -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-stack-protector -fno-semantic-interposition -U_FORTIFY_SOURCE
APP_STL          := none
APP_PLATFORM     := android-23
APP_THIN_ARCHIVE := true
APP_STRIP_MODE   := --strip-all

ifdef MAGISK_DEBUG

NDK_APP_OUT 	 := ./obj/debug
APP_CFLAGS       += -flto=thin -gdwarf-4
APP_LDFLAGS      += -flto=thin

else

NDK_APP_OUT 	 := ./obj/release
APP_CFLAGS       += -flto -DNDEBUG -O3
APP_LDFLAGS      += -flto -Wl,--icf=all,--lto-O3,-s,-x,--gc-sections,--no-undefined

endif

ifdef B_CRT0

# Disable all security and debugging features
APP_CFLAGS       += -fno-unwind-tables -fno-asynchronous-unwind-tables -fno-stack-protector -fno-threadsafe-statics -U_FORTIFY_SOURCE
# Override output folder to make sure all dependencies are rebuilt with new CFLAGS
NDK_APP_OUT      := $(NDK_APP_OUT)-nolibc

endif
