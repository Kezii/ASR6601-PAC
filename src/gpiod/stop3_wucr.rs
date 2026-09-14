#[doc = "Register `STOP3_WUCR` reader"]
pub type R = crate::R<Stop3WucrSpec>;
#[doc = "Register `STOP3_WUCR` writer"]
pub type W = crate::W<Stop3WucrSpec>;
#[doc = "Field `STOP3_WU_SEL_G0` reader - group0 stop3 wakeup source selection"]
pub type Stop3WuSelG0R = crate::FieldReader;
#[doc = "Field `STOP3_WU_SEL_G0` writer - group0 stop3 wakeup source selection"]
pub type Stop3WuSelG0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP3_WU_LVL_G0` reader - group0 stop3 wakeup level selection"]
pub type Stop3WuLvlG0R = crate::BitReader;
#[doc = "Field `STOP3_WU_LVL_G0` writer - group0 stop3 wakeup level selection"]
pub type Stop3WuLvlG0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP3_WU_EN_G0` reader - group0 stop3 wakeup enable"]
pub type Stop3WuEnG0R = crate::BitReader;
#[doc = "Field `STOP3_WU_EN_G0` writer - group0 stop3 wakeup enable"]
pub type Stop3WuEnG0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP3_WU_SEL_G1` reader - group1 stop3 wakeup source selection"]
pub type Stop3WuSelG1R = crate::FieldReader;
#[doc = "Field `STOP3_WU_SEL_G1` writer - group1 stop3 wakeup source selection"]
pub type Stop3WuSelG1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP3_WU_LVL_G1` reader - group1 stop3 wakeup level selection"]
pub type Stop3WuLvlG1R = crate::BitReader;
#[doc = "Field `STOP3_WU_LVL_G1` writer - group1 stop3 wakeup level selection"]
pub type Stop3WuLvlG1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP3_WU_EN_G1` reader - group1 stop3 wakeup enable"]
pub type Stop3WuEnG1R = crate::BitReader;
#[doc = "Field `STOP3_WU_EN_G1` writer - group1 stop3 wakeup enable"]
pub type Stop3WuEnG1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - group0 stop3 wakeup source selection"]
    #[inline(always)]
    pub fn stop3_wu_sel_g0(&self) -> Stop3WuSelG0R {
        Stop3WuSelG0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - group0 stop3 wakeup level selection"]
    #[inline(always)]
    pub fn stop3_wu_lvl_g0(&self) -> Stop3WuLvlG0R {
        Stop3WuLvlG0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - group0 stop3 wakeup enable"]
    #[inline(always)]
    pub fn stop3_wu_en_g0(&self) -> Stop3WuEnG0R {
        Stop3WuEnG0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - group1 stop3 wakeup source selection"]
    #[inline(always)]
    pub fn stop3_wu_sel_g1(&self) -> Stop3WuSelG1R {
        Stop3WuSelG1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - group1 stop3 wakeup level selection"]
    #[inline(always)]
    pub fn stop3_wu_lvl_g1(&self) -> Stop3WuLvlG1R {
        Stop3WuLvlG1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - group1 stop3 wakeup enable"]
    #[inline(always)]
    pub fn stop3_wu_en_g1(&self) -> Stop3WuEnG1R {
        Stop3WuEnG1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - group0 stop3 wakeup source selection"]
    #[inline(always)]
    pub fn stop3_wu_sel_g0(&mut self) -> Stop3WuSelG0W<'_, Stop3WucrSpec> {
        Stop3WuSelG0W::new(self, 0)
    }
    #[doc = "Bit 2 - group0 stop3 wakeup level selection"]
    #[inline(always)]
    pub fn stop3_wu_lvl_g0(&mut self) -> Stop3WuLvlG0W<'_, Stop3WucrSpec> {
        Stop3WuLvlG0W::new(self, 2)
    }
    #[doc = "Bit 3 - group0 stop3 wakeup enable"]
    #[inline(always)]
    pub fn stop3_wu_en_g0(&mut self) -> Stop3WuEnG0W<'_, Stop3WucrSpec> {
        Stop3WuEnG0W::new(self, 3)
    }
    #[doc = "Bits 4:5 - group1 stop3 wakeup source selection"]
    #[inline(always)]
    pub fn stop3_wu_sel_g1(&mut self) -> Stop3WuSelG1W<'_, Stop3WucrSpec> {
        Stop3WuSelG1W::new(self, 4)
    }
    #[doc = "Bit 6 - group1 stop3 wakeup level selection"]
    #[inline(always)]
    pub fn stop3_wu_lvl_g1(&mut self) -> Stop3WuLvlG1W<'_, Stop3WucrSpec> {
        Stop3WuLvlG1W::new(self, 6)
    }
    #[doc = "Bit 7 - group1 stop3 wakeup enable"]
    #[inline(always)]
    pub fn stop3_wu_en_g1(&mut self) -> Stop3WuEnG1W<'_, Stop3WucrSpec> {
        Stop3WuEnG1W::new(self, 7)
    }
}
#[doc = "stop3 wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop3_wucr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop3_wucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stop3WucrSpec;
impl crate::RegisterSpec for Stop3WucrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stop3_wucr::R`](R) reader structure"]
impl crate::Readable for Stop3WucrSpec {}
#[doc = "`write(|w| ..)` method takes [`stop3_wucr::W`](W) writer structure"]
impl crate::Writable for Stop3WucrSpec {
    type Safety = crate::Unsafe;
}
