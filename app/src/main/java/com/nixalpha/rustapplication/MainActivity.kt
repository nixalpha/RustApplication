package com.nixalpha.rustapplication

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import com.nixalpha.rustapplication.ui.theme.RustApplicationTheme

import com.nixalpha.rustapplication.lib.Session

class MainActivity : ComponentActivity() {
    companion object {
        init {
            System.loadLibrary("rust_lib")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            RustApplicationTheme {
                // A surface container using the 'background' color from the theme
                Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colors.background) {
                    Greeting("Android")
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String) {
    val context = LocalContext.current
    val libDir = context.applicationInfo.dataDir
    Log.d("DIR", libDir)

    val session = Session(2)
    val num = session.add_and1(1)

    Log.d("DIR", Session.lspath(libDir))

    val greetJni = Session.greet("jni")
    Text(text = greetJni)
}

//@Preview(showBackground = true)
//@Composable
//fun DefaultPreview() {
//    RustApplicationTheme {
//        Greeting("Android")
//    }
//}