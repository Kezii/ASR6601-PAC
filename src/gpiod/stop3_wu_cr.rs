#[doc = "Register `STOP3_WU_CR` reader"]
pub type R = crate::R<Stop3WuCrSpec>;
#[doc = "Register `STOP3_WU_CR` writer"]
pub type W = crate::W<Stop3WuCrSpec>;
#[doc = "group0 stop3 wakeup source selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stop3WuSelG0 {
    #[doc = "0: Pin 0"]
    Pin0 = 0,
    #[doc = "1: Pin 1"]
    Pin1 = 1,
    #[doc = "2: Pin 2"]
    Pin2 = 2,
    #[doc = "3: Pin 3"]
    Pin3 = 3,
}
impl From<Stop3WuSelG0> for u8 {
    #[inline(always)]
    fn from(variant: Stop3WuSelG0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stop3WuSelG0 {
    type Ux = u8;
}
impl crate::IsEnum for Stop3WuSelG0 {}
#[doc = "Field `STOP3_WU_SEL_G0` reader - group0 stop3 wakeup source selection"]
pub type Stop3WuSelG0R = crate::FieldReader<Stop3WuSelG0>;
impl Stop3WuSelG0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop3WuSelG0 {
        match self.bits {
            0 => Stop3WuSelG0::Pin0,
            1 => Stop3WuSelG0::Pin1,
            2 => Stop3WuSelG0::Pin2,
            3 => Stop3WuSelG0::Pin3,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == Stop3WuSelG0::Pin0
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == Stop3WuSelG0::Pin1
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == Stop3WuSelG0::Pin2
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == Stop3WuSelG0::Pin3
    }
}
#[doc = "Field `STOP3_WU_SEL_G0` writer - group0 stop3 wakeup source selection"]
pub type Stop3WuSelG0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Stop3WuSelG0, crate::Safe>;
impl<'a, REG> Stop3WuSelG0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3WuSelG0::Pin0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3WuSelG0::Pin1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3WuSelG0::Pin2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3WuSelG0::Pin3)
    }
}
#[doc = "Field `STOP3_WU_LVL_G0` reader - group0 stop3 wakeup level selection"]
pub type Stop3WuLvlG0R = crate::BitReader;
#[doc = "Field `STOP3_WU_LVL_G0` writer - group0 stop3 wakeup level selection"]
pub type Stop3WuLvlG0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP3_WU_EN_G0` reader - group0 stop3 wakeup enable"]
pub type Stop3WuEnG0R = crate::BitReader;
#[doc = "Field `STOP3_WU_EN_G0` writer - group0 stop3 wakeup enable"]
pub type Stop3WuEnG0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "group1 stop3 wakeup source selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stop3WuSelG1 {
    #[doc = "0: Pin 4"]
    Pin4 = 0,
    #[doc = "1: Pin 5"]
    Pin5 = 1,
    #[doc = "2: Pin 6"]
    Pin6 = 2,
    #[doc = "3: Pin 7"]
    Pin7 = 3,
}
impl From<Stop3WuSelG1> for u8 {
    #[inline(always)]
    fn from(variant: Stop3WuSelG1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stop3WuSelG1 {
    type Ux = u8;
}
impl crate::IsEnum for Stop3WuSelG1 {}
#[doc = "Field `STOP3_WU_SEL_G1` reader - group1 stop3 wakeup source selection"]
pub type Stop3WuSelG1R = crate::FieldReader<Stop3WuSelG1>;
impl Stop3WuSelG1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop3WuSelG1 {
        match self.bits {
            0 => Stop3WuSelG1::Pin4,
            1 => Stop3WuSelG1::Pin5,
            2 => Stop3WuSelG1::Pin6,
            3 => Stop3WuSelG1::Pin7,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == Stop3WuSelG1::Pin4
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == Stop3WuSelG1::Pin5
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == Stop3WuSelG1::Pin6
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == Stop3WuSelG1::Pin7
    }
}
#[doc = "Field `STOP3_WU_SEL_G1` writer - group1 stop3 wakeup source selection"]
pub type Stop3WuSelG1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Stop3WuSelG1, crate::Safe>;
impl<'a, REG> Stop3WuSelG1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3WuSelG1::Pin4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3WuSelG1::Pin5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3WuSelG1::Pin6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut crate::W<REG> {
        self.variant(Stop3WuSelG1::Pin7)
    }
}
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
    pub fn stop3_wu_sel_g0(&mut self) -> Stop3WuSelG0W<'_, Stop3WuCrSpec> {
        Stop3WuSelG0W::new(self, 0)
    }
    #[doc = "Bit 2 - group0 stop3 wakeup level selection"]
    #[inline(always)]
    pub fn stop3_wu_lvl_g0(&mut self) -> Stop3WuLvlG0W<'_, Stop3WuCrSpec> {
        Stop3WuLvlG0W::new(self, 2)
    }
    #[doc = "Bit 3 - group0 stop3 wakeup enable"]
    #[inline(always)]
    pub fn stop3_wu_en_g0(&mut self) -> Stop3WuEnG0W<'_, Stop3WuCrSpec> {
        Stop3WuEnG0W::new(self, 3)
    }
    #[doc = "Bits 4:5 - group1 stop3 wakeup source selection"]
    #[inline(always)]
    pub fn stop3_wu_sel_g1(&mut self) -> Stop3WuSelG1W<'_, Stop3WuCrSpec> {
        Stop3WuSelG1W::new(self, 4)
    }
    #[doc = "Bit 6 - group1 stop3 wakeup level selection"]
    #[inline(always)]
    pub fn stop3_wu_lvl_g1(&mut self) -> Stop3WuLvlG1W<'_, Stop3WuCrSpec> {
        Stop3WuLvlG1W::new(self, 6)
    }
    #[doc = "Bit 7 - group1 stop3 wakeup enable"]
    #[inline(always)]
    pub fn stop3_wu_en_g1(&mut self) -> Stop3WuEnG1W<'_, Stop3WuCrSpec> {
        Stop3WuEnG1W::new(self, 7)
    }
}
#[doc = "stop3 wakeup control register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop3_wu_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop3_wu_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stop3WuCrSpec;
impl crate::RegisterSpec for Stop3WuCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stop3_wu_cr::R`](R) reader structure"]
impl crate::Readable for Stop3WuCrSpec {}
#[doc = "`write(|w| ..)` method takes [`stop3_wu_cr::W`](W) writer structure"]
impl crate::Writable for Stop3WuCrSpec {
    type Safety = crate::Unsafe;
}
