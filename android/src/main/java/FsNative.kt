package com.plugin.fsnative

import android.app.Activity
import android.content.Context
import android.net.Uri
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

import java.io.BufferedReader
import java.io.InputStreamReader
import java.nio.charset.StandardCharsets

@InvokeArg
class ReadFileArgs (
    val uri: String
)

@TauriPlugin
class FsNativePlugin(private val activity: Activity): Plugin(activity) {
    @Command
    fun read_to_string(invoke: Invoke) {
        val args = invoke.parseArgs(ReadFileArgs::class.java)

        try {
            val csvContent = readTextFromUri(activity.getApplicationContext(), Uri.parse(args.uri))
            val ret = JSObject()
            ret.put("content", csvContent)
            invoke.resolve(ret)
        } catch (e: Exception) {
            invoke.reject("READ_ERROR", "Failed to read CSV file: ${e.message}", JSObject())
        }
    }

    private fun readTextFromUri(context: Context, uri: Uri): String {
        context.contentResolver.openInputStream(uri)?.use { inputStream ->
            InputStreamReader(inputStream, StandardCharsets.UTF_8).use { reader ->
                BufferedReader(reader).use { bufferedReader ->
                    return bufferedReader.readText()
                }
            }
        } ?: throw IllegalArgumentException("Unable to open input stream for URI: $uri")
    }
}
