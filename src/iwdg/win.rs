#[doc = "Register `WIN` reader"]
pub type R = crate::R<WinSpec>;
#[doc = "Register `WIN` writer"]
pub type W = crate::W<WinSpec>;
#[doc = "Field `WIN` reader - Window value, 12-bit (vendor SDK clamps to 0x0FFF)"]
pub type WinR = crate::FieldReader<u16>;
#[doc = "Field `WIN` writer - Window value, 12-bit (vendor SDK clamps to 0x0FFF)"]
pub type WinW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Window value, 12-bit (vendor SDK clamps to 0x0FFF)"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window value, 12-bit (vendor SDK clamps to 0x0FFF)"]
    #[inline(always)]
    pub fn win(&mut self) -> WinW<'_, WinSpec> {
        WinW::new(self, 0)
    }
}
#[doc = "window value register\n\nYou can [`read`](crate::Reg::read) this register and get [`win::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinSpec;
impl crate::RegisterSpec for WinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win::R`](R) reader structure"]
impl crate::Readable for WinSpec {}
#[doc = "`write(|w| ..)` method takes [`win::W`](W) writer structure"]
impl crate::Writable for WinSpec {
    type Safety = crate::Unsafe;
}
