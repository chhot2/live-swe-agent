//! Media Player Module
//!
//! Provides enhanced media playback functionality.

use serde::{Deserialize, Serialize};

pub struct MediaPlayer {
    config: MediaPlayerConfig,
    current_media: Option<MediaInfo>,
    playlist: Vec<MediaInfo>,
    history: Vec<MediaInfo>,
}

/// Media player configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a MediaPlayerConfig.
pub struct MediaPlayerConfig {
    /// Default volume (0.0 - 1.0)
    pub default_volume: f64,
    /// Enable picture-in-picture
    pub pip_enabled: bool,
    /// Enable background playback
    pub background_playback: bool,
    /// Default playback speed
    pub default_speed: f64,
    /// Enable auto quality
    pub auto_quality: bool,
    /// Preferred quality
    pub preferred_quality: VideoQuality,
    /// Enable subtitles by default
    pub subtitles_enabled: bool,
    /// Preferred subtitle language
    pub subtitle_language: String,
    /// Enable media keys
    pub media_keys_enabled: bool,
    /// Enable skip silence
    pub skip_silence: bool,
}

impl Default for MediaPlayerConfig {
    fn default() -> Self {
        Self {
            default_volume: 1.0,
            pip_enabled: true,
            background_playback: true,
            default_speed: 1.0,
            auto_quality: true,
            preferred_quality: VideoQuality::Auto,
            subtitles_enabled: false,
            subtitle_language: "en".to_string(),
            media_keys_enabled: true,
            skip_silence: false,
        }
    }
}

/// Video quality options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Enumeration of VideoQuality variants.
pub enum VideoQuality {
    Auto,
    Quality144p,
    Quality240p,
    Quality360p,
    Quality480p,
    Quality720p,
    Quality1080p,
    Quality1440p,
    Quality4k,
    Quality8k,
}

/// Media information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a MediaInfo.
pub struct MediaInfo {
    pub url: String,
    pub title: String,
    pub media_type: MediaType,
    pub duration_seconds: Option<f64>,
    pub thumbnail: Option<String>,
    pub source: Option<String>,
    pub quality_options: Vec<VideoQuality>,
}

/// Media type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Enumeration of MediaType variants.
pub enum MediaType {
    Video,
    Audio,
    Stream,
    Playlist,
}

impl MediaPlayer {
    /// Creates a new new.
    pub fn new() -> Self {
        Self {
            config: MediaPlayerConfig::default(),
            current_media: None,
            playlist: Vec::new(),
            history: Vec::new(),
        }
    }

    /// Set current media
    /// Set the current media to play
    ///
    /// # Arguments
    /// * `media` - The media information
    pub fn set_media(&mut self, media: MediaInfo) {
        if let Some(current) = self.current_media.take() {
            self.history.push(current);
        }
        self.current_media = Some(media);
    }

    /// Add to playlist
    /// Add media to the playlist
    ///
    /// # Arguments
    /// * `media` - The media to add
    pub fn add_to_playlist(&mut self, media: MediaInfo) {
        self.playlist.push(media);
    }

    /// Get current media
    /// Get the currently playing media
    pub fn get_current(&self) -> Option<&MediaInfo> {
        self.current_media.as_ref()
    }

    /// Get playlist
    /// Get the current playlist
    pub fn get_playlist(&self) -> &[MediaInfo] {
        &self.playlist
    }

    /// Play next in playlist
    /// Move to the next item in the playlist
    pub fn play_next(&mut self) -> Option<&MediaInfo> {
        if !self.playlist.is_empty() {
            if let Some(current) = self.current_media.take() {
                self.history.push(current);
            }
            self.current_media = Some(self.playlist.remove(0));
        }
        self.current_media.as_ref()
    }

    /// Play previous from history
    /// Move to the previous item in the playlist
    pub fn previous(&mut self) -> Option<&MediaInfo> {
        if let Some(prev) = self.history.pop() {
            if let Some(current) = self.current_media.take() {
                self.playlist.insert(0, current);
            }
            self.current_media = Some(prev);
        }
        self.current_media.as_ref()
    }

    /// Generate enhanced player JavaScript
    /// Generate JavaScript for the media player UI
    pub fn generate_player_script(&self) -> String {
        format!(
            r#"
(function() {{
    const config = {{
        defaultVolume: {volume},
        defaultSpeed: {speed},
        pipEnabled: {pip},
        backgroundPlayback: {background},
        skipSilence: {skip_silence}
    }};
    
    // Enhanced video/audio controls
    document.querySelectorAll('video, audio').forEach(media => {{
        media.volume = config.defaultVolume;
        media.playbackRate = config.defaultSpeed;
        
        // Add keyboard shortcuts
        document.addEventListener('keydown', (e) => {{
            if (e.target.tagName !== 'INPUT' && e.target.tagName !== 'TEXTAREA') {{
                switch(e.key) {{
                    case ' ':
                        e.preventDefault();
                        media.paused ? media.play() : media.pause();
                        break;
                    case 'ArrowLeft':
                        media.currentTime -= 10;
                        break;
                    case 'ArrowRight':
                        media.currentTime += 10;
                        break;
                    case 'ArrowUp':
                        media.volume = Math.min(1, media.volume + 0.1);
                        break;
                    case 'ArrowDown':
                        media.volume = Math.max(0, media.volume - 0.1);
                        break;
                    case 'f':
                        if (document.fullscreenElement) {{
                            document.exitFullscreen();
                        }} else {{
                            media.requestFullscreen();
                        }}
                        break;
                    case 'p':
                        if (config.pipEnabled && document.pictureInPictureEnabled) {{
                            if (document.pictureInPictureElement) {{
                                document.exitPictureInPicture();
                            }} else {{
                                media.requestPictureInPicture();
                            }}
                        }}
                        break;
                }}
            }}
        }});
    }});
    
    console.log('[MediaPlayer] Enhanced controls active');
}})();
"#,
            volume = self.config.default_volume,
            speed = self.config.default_speed,
            pip = self.config.pip_enabled,
            background = self.config.background_playback,
            skip_silence = self.config.skip_silence,
        )
    }

    /// Update configuration
    pub fn set_config(&mut self, config: MediaPlayerConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &MediaPlayerConfig {
        &self.config
    }
}

impl Default for MediaPlayer {
    fn default() -> Self {
        Self::new()
    }
}
