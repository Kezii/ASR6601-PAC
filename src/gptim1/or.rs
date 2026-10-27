#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Channel 2 remapping"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ti2Rmp {
    #[doc = "0: IOM"]
    Iom = 0,
    #[doc = "1: TIM3_CH1"]
    Tim3Ch1 = 1,
}
impl From<Ti2Rmp> for u8 {
    #[inline(always)]
    fn from(variant: Ti2Rmp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ti2Rmp {
    type Ux = u8;
}
impl crate::IsEnum for Ti2Rmp {}
#[doc = "Field `TI2_RMP` reader - Channel 2 remapping"]
pub type Ti2RmpR = crate::FieldReader<Ti2Rmp>;
impl Ti2RmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ti2Rmp> {
        match self.bits {
            0 => Some(Ti2Rmp::Iom),
            1 => Some(Ti2Rmp::Tim3Ch1),
            _ => None,
        }
    }
    #[doc = "IOM"]
    #[inline(always)]
    pub fn is_iom(&self) -> bool {
        *self == Ti2Rmp::Iom
    }
    #[doc = "TIM3_CH1"]
    #[inline(always)]
    pub fn is_tim3_ch1(&self) -> bool {
        *self == Ti2Rmp::Tim3Ch1
    }
}
#[doc = "Field `TI2_RMP` writer - Channel 2 remapping"]
pub type Ti2RmpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ti2Rmp>;
impl<'a, REG> Ti2RmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IOM"]
    #[inline(always)]
    pub fn iom(self) -> &'a mut crate::W<REG> {
        self.variant(Ti2Rmp::Iom)
    }
    #[doc = "TIM3_CH1"]
    #[inline(always)]
    pub fn tim3_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(Ti2Rmp::Tim3Ch1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 2 remapping"]
    #[inline(always)]
    pub fn ti2_rmp(&self) -> Ti2RmpR {
        Ti2RmpR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 2 remapping"]
    #[inline(always)]
    pub fn ti2_rmp(&mut self) -> Ti2RmpW<'_, OrSpec> {
        Ti2RmpW::new(self, 0)
    }
}
#[doc = "GPTIM option register (GPTIM1 remap layout)\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
}
