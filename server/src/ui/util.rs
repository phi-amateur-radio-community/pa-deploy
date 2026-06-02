// =====================================================================
// Copyright (c) 2026 Phiarc Team and St Rangeset
// Licensed under the GNU General Public License Version 3.0 or later
// https://github.com/phi-amateur-radio-community/pa-deploy
// =====================================================================
// Path /server/src/ui/util.rs
// Tools for core

impl ScreenData {
    fn get_len_ptr(&self) -> (usize, usize) {
        match self.detail_ptr {
            Some(ptr) => (self.map_cache.get_len(), ptr),
            None => (self.config.get_len(), self.ptr),
        }
    }

    fn limit_move(&mut self, is_up: bool) {
        if is_up {
            if self.ptr != 0 {
                self.ptr -= 1;
            }
        } else {
        let (len, ptr) = self.get_len_ptr();
        if ptr < len - 1 {
            self.ptr += 1;
        }
    }
    }
}
