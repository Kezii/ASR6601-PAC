#[doc = "Register `MOSI_CR` reader"]
pub type R = crate::R<MosiCrSpec>;
#[doc = "Register `MOSI_CR` writer"]
pub type W = crate::W<MosiCrSpec>;
#[doc = "mosi control bit"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegMosi {
    #[doc = "0: pull down mosi pin"]
    PullDown = 0,
    #[doc = "1: pull up mosi pin"]
    PullUp = 1,
}
impl From<RegMosi> for bool {
    #[inline(always)]
    fn from(variant: RegMosi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REG_MOSI` reader - mosi control bit"]
pub type RegMosiR = crate::BitReader<RegMosi>;
impl RegMosiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RegMosi {
        match self.bits {
            false => RegMosi::PullDown,
            true => RegMosi::PullUp,
        }
    }
    #[doc = "pull down mosi pin"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == RegMosi::PullDown
    }
    #[doc = "pull up mosi pin"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == RegMosi::PullUp
    }
}
#[doc = "Field `REG_MOSI` writer - mosi control bit"]
pub type RegMosiW<'a, REG> = crate::BitWriter<'a, REG, RegMosi>;
impl<'a, REG> RegMosiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "pull down mosi pin"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(RegMosi::PullDown)
    }
    #[doc = "pull up mosi pin"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(RegMosi::PullUp)
    }
}
impl R {
    #[doc = "Bit 0 - mosi control bit"]
    #[inline(always)]
    pub fn reg_mosi(&self) -> RegMosiR {
        RegMosiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mosi control bit"]
    #[inline(always)]
    pub fn reg_mosi(&mut self) -> RegMosiW<'_, MosiCrSpec> {
        RegMosiW::new(self, 0)
    }
}
#[doc = "mosi control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi_cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MosiCrSpec;
impl crate::RegisterSpec for MosiCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mosi_cr::R`](R) reader structure"]
impl crate::Readable for MosiCrSpec {}
#[doc = "`write(|w| ..)` method takes [`mosi_cr::W`](W) writer structure"]
impl crate::Writable for MosiCrSpec {
    type Safety = crate::Unsafe;
}
