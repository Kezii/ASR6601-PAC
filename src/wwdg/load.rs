#[doc = "Register `LOAD` reader"]
pub type R = crate::R<LoadSpec>;
#[doc = "Register `LOAD` writer"]
pub type W = crate::W<LoadSpec>;
#[doc = "Field `LOAD` reader - Watchdog reload value, 32-bit"]
pub type LoadR = crate::FieldReader<u32>;
#[doc = "Field `LOAD` writer - Watchdog reload value, 32-bit"]
pub type LoadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Watchdog reload value, 32-bit"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog reload value, 32-bit"]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, LoadSpec> {
        LoadW::new(self, 0)
    }
}
#[doc = "load register\n\nYou can [`read`](crate::Reg::read) this register and get [`load::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadSpec;
impl crate::RegisterSpec for LoadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load::R`](R) reader structure"]
impl crate::Readable for LoadSpec {}
#[doc = "`write(|w| ..)` method takes [`load::W`](W) writer structure"]
impl crate::Writable for LoadSpec {
    type Safety = crate::Unsafe;
}
