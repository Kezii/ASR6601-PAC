#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `IRQ_DIG_INT_EN` reader - irq dig high level interrupt enable, bit per irq dig line"]
pub type IrqDigIntEnR = crate::FieldReader;
#[doc = "Field `IRQ_DIG_INT_EN` writer - irq dig high level interrupt enable, bit per irq dig line"]
pub type IrqDigIntEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "sck/mosi/miso source selection for rf trx"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SckMosiSel {
    #[doc = "0: from LORAC_SCK_CR, LORAC_MOSI_CR and LORAC_MISO_SR"]
    Reg = 0,
    #[doc = "1: from internal ssp of LORAC"]
    InternalSsp = 1,
}
impl From<SckMosiSel> for bool {
    #[inline(always)]
    fn from(variant: SckMosiSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCK_MOSI_SEL` reader - sck/mosi/miso source selection for rf trx"]
pub type SckMosiSelR = crate::BitReader<SckMosiSel>;
impl SckMosiSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SckMosiSel {
        match self.bits {
            false => SckMosiSel::Reg,
            true => SckMosiSel::InternalSsp,
        }
    }
    #[doc = "from LORAC_SCK_CR, LORAC_MOSI_CR and LORAC_MISO_SR"]
    #[inline(always)]
    pub fn is_reg(&self) -> bool {
        *self == SckMosiSel::Reg
    }
    #[doc = "from internal ssp of LORAC"]
    #[inline(always)]
    pub fn is_internal_ssp(&self) -> bool {
        *self == SckMosiSel::InternalSsp
    }
}
#[doc = "Field `SCK_MOSI_SEL` writer - sck/mosi/miso source selection for rf trx"]
pub type SckMosiSelW<'a, REG> = crate::BitWriter<'a, REG, SckMosiSel>;
impl<'a, REG> SckMosiSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "from LORAC_SCK_CR, LORAC_MOSI_CR and LORAC_MISO_SR"]
    #[inline(always)]
    pub fn reg(self) -> &'a mut crate::W<REG> {
        self.variant(SckMosiSel::Reg)
    }
    #[doc = "from internal ssp of LORAC"]
    #[inline(always)]
    pub fn internal_ssp(self) -> &'a mut crate::W<REG> {
        self.variant(SckMosiSel::InternalSsp)
    }
}
#[doc = "nss source selection for rf trx"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NssSel {
    #[doc = "0: from register LORAC_NSS_CR"]
    Reg = 0,
    #[doc = "1: from internal ssp of LORAC"]
    InternalSsp = 1,
}
impl From<NssSel> for bool {
    #[inline(always)]
    fn from(variant: NssSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSS_SEL` reader - nss source selection for rf trx"]
pub type NssSelR = crate::BitReader<NssSel>;
impl NssSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NssSel {
        match self.bits {
            false => NssSel::Reg,
            true => NssSel::InternalSsp,
        }
    }
    #[doc = "from register LORAC_NSS_CR"]
    #[inline(always)]
    pub fn is_reg(&self) -> bool {
        *self == NssSel::Reg
    }
    #[doc = "from internal ssp of LORAC"]
    #[inline(always)]
    pub fn is_internal_ssp(&self) -> bool {
        *self == NssSel::InternalSsp
    }
}
#[doc = "Field `NSS_SEL` writer - nss source selection for rf trx"]
pub type NssSelW<'a, REG> = crate::BitWriter<'a, REG, NssSel>;
impl<'a, REG> NssSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "from register LORAC_NSS_CR"]
    #[inline(always)]
    pub fn reg(self) -> &'a mut crate::W<REG> {
        self.variant(NssSel::Reg)
    }
    #[doc = "from internal ssp of LORAC"]
    #[inline(always)]
    pub fn internal_ssp(self) -> &'a mut crate::W<REG> {
        self.variant(NssSel::InternalSsp)
    }
}
impl R {
    #[doc = "Bits 5:7 - irq dig high level interrupt enable, bit per irq dig line"]
    #[inline(always)]
    pub fn irq_dig_int_en(&self) -> IrqDigIntEnR {
        IrqDigIntEnR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 9 - sck/mosi/miso source selection for rf trx"]
    #[inline(always)]
    pub fn sck_mosi_sel(&self) -> SckMosiSelR {
        SckMosiSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - nss source selection for rf trx"]
    #[inline(always)]
    pub fn nss_sel(&self) -> NssSelR {
        NssSelR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - irq dig high level interrupt enable, bit per irq dig line"]
    #[inline(always)]
    pub fn irq_dig_int_en(&mut self) -> IrqDigIntEnW<'_, Cr0Spec> {
        IrqDigIntEnW::new(self, 5)
    }
    #[doc = "Bit 9 - sck/mosi/miso source selection for rf trx"]
    #[inline(always)]
    pub fn sck_mosi_sel(&mut self) -> SckMosiSelW<'_, Cr0Spec> {
        SckMosiSelW::new(self, 9)
    }
    #[doc = "Bit 10 - nss source selection for rf trx"]
    #[inline(always)]
    pub fn nss_sel(&mut self) -> NssSelW<'_, Cr0Spec> {
        NssSelW::new(self, 10)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
