// Copyright (c) 2015 T. Okub
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use vlc_sys as sys;
use crate::Instance;
use crate::MediaList;
use crate::MediaPlayer;
use crate::EventManager;

// A LibVLC media list player plays multiple media from a medialist
pub struct MediaListPlayer {
    pub(crate) ptr: *mut sys::libvlc_media_list_player_t,
}

unsafe impl Send for MediaListPlayer {}

impl MediaListPlayer {
    // Create an empty Media Player object
    pub fn new(instance: &Instance) -> Option<MediaListPlayer> {
        unsafe{
            let p = sys::libvlc_media_list_player_new(instance.ptr);

            if p.is_null() {
                return None;
            }
            Some(MediaListPlayer{ptr: p})
        }
    }

    // Set the media player that will be used by the media_list_player.
    pub fn set_media_player(&self, mdp: &MediaPlayer) {
        unsafe{ sys::libvlc_media_list_player_set_media_player(self.ptr, mdp.ptr) };
    }

    // Get the media player used by the media_list_player.
    pub fn get_media(&self) -> Option<MediaPlayer> {
        let p = unsafe{ sys::libvlc_media_list_player_get_media_player(self.ptr) };
        if p.is_null() {
            None
        }else{
            Some(MediaPlayer{ptr: p})
        }
    }

    // Set the media list that will be used by the media_list_player.
    pub fn set_media_list(&self, ml: &MediaList) {
        unsafe{ sys::libvlc_media_list_player_set_media_list(self.ptr, ml.ptr) };
    }

    // Get the Event Manager from which the media player send event.
    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe{
            let p = sys::libvlc_media_list_player_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager{ptr: p, _phantomdata: ::std::marker::PhantomData}
        }
    }

    // is_playing
    pub fn is_playing(&self) -> bool {
        if unsafe{ sys::libvlc_media_list_player_is_playing(self.ptr) } == 0 {
            false
        }else{
            true
        }
    }

    // Play
    pub fn play(&self) -> Result<(), ()> {
        unsafe { 
            sys::libvlc_media_list_player_play(self.ptr);
            Ok(())
        }
    }

    // Pause or resume (no effect if there is no media)
    pub fn set_pause(&self, do_pause: bool) {
        unsafe{ sys::libvlc_media_list_player_set_pause(self.ptr, if do_pause {1} else {0}) };
    }

    // Toggle pause (no effect if there is no media)
    pub fn pause(&self) {
        unsafe{ sys::libvlc_media_list_player_pause(self.ptr) };
    }

    // Stop (no effect if there is no media)
    pub fn stop(&self) {
        unsafe{ sys::libvlc_media_list_player_stop(self.ptr) };
    }

    // Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_media_list_player_t {
        self.ptr
    }
}

impl Drop for MediaListPlayer {
    fn drop(&mut self) {
        unsafe{ sys::libvlc_media_list_player_release(self.ptr) };
    }
}
