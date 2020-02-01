#[allow(dead_code)]
mod lib_example;

use jni::objects::{JObject, JString};
use jni::sys::{jarray, jobjectArray, jstring};
use jni::JNIEnv;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};

pub static FILES_PATH: &str = "/data/user/0/com.example.android/files/";

// Return a i32 from another i32
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_returni32(
    _env: JNIEnv,
    _: JObject,
    int: i32,
) -> i32 {
    lib_example::int_times_2(int)
}

// Return a bool (true when a specific file in our folder exists)
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getFileStatus(
    _env: JNIEnv,
    _: JObject,
) -> bool {
    lib_example::get_file_status(&FILES_PATH)
}

// Return JString from JString
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_returnJString(
    env: JNIEnv,
    _: JObject,
    old_jstring: JString,
) -> jstring {
    // convert JString to CString and append it to our JNIEnv
    env.new_string(lib_example::return_string(
        CString::from(CStr::from_ptr(
            env.get_string(old_jstring).unwrap().as_ptr(),
        ))
        .to_str()
        .unwrap()
        .to_string(),
    ))
    .unwrap()
    .into_inner()
}

// Return Array from Vector from JString
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_returnJArrayfromJString(
    env: JNIEnv,
    _: JObject,
    jstring_sentence: JString,
) -> jarray {
    let new_vector = lib_example::split_string_into_words(
        CString::from(CStr::from_ptr(
            env.get_string(jstring_sentence).unwrap().as_ptr(),
        ))
        .to_str()
        .unwrap()
        .to_string(),
    );
    // Initialize our array with the length of the vector. JArray(Length, Class, Initial Value)
    let array: jobjectArray = env
        .new_object_array(
            i32::try_from(new_vector.len()).unwrap(),
            env.find_class("java/lang/String").unwrap(),
            *env.new_string("").unwrap(),
        )
        .unwrap();
    let mut i = 0;
    // Edit every Item of the Array to give it the values we want
    for item in &new_vector {
        env.set_object_array_element(
            array,
            i,
            *env.new_string(item.to_owned()).unwrap().to_owned(),
        )
        .expect("Could not perform set_object_array_element on array element.");
        i += 1;
    }
    array
}
