#[doc = "Register `TIMING_CFG` reader"]
pub type R = crate::R<TimingCfgSpec>;
#[doc = "Register `TIMING_CFG` writer"]
pub type W = crate::W<TimingCfgSpec>;
#[doc = "Field `READ_NUM` reader - Read num"]
pub type ReadNumR = crate::FieldReader;
#[doc = "Field `READ_NUM` writer - Read num"]
pub type ReadNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 16:19 - Read num"]
    #[inline(always)]
    pub fn read_num(&self) -> ReadNumR {
        ReadNumR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Read num"]
    #[inline(always)]
    pub fn read_num(&mut self) -> ReadNumW<'_, TimingCfgSpec> {
        ReadNumW::new(self, 16)
    }
}
#[doc = "timing config register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing_cfg::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimingCfgSpec;
impl crate::RegisterSpec for TimingCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing_cfg::R`](R) reader structure"]
impl crate::Readable for TimingCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`timing_cfg::W`](W) writer structure"]
impl crate::Writable for TimingCfgSpec {
    type Safety = crate::Unsafe;
}
